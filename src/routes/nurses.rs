use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{self, addresses, nurses, skills, users},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete, availabilities),
    components(schemas(
        Nurse,
        SkilledNurse,
        NurseRecord,
        UpdateNurse,
        NewNurse,
        User,
        Availability,
        crate::pagination::PaginatedNurses,
        crate::pagination::PaginatedAvailabilities,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/nurses")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
        .service(availabilities)
}

#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of nurses", body = PaginatedNurses),
    ),
    tag = "nurses"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();
    let p3 = pool.clone();

    // Get nurses
    let nurses: Vec<Nurse> = list!(nurses, pool, query, users, addresses);
    // Get total of nurses
    let total = total!(nurses, p2);

    // Get database records
    let nurses_records: Vec<_> = nurses.iter().map(|n| n.nurse).collect();

    // Get skills and group by nurse
    let res = LNurseSkill::belonging_to(&nurses_records)
        .inner_join(skills::table)
        .load::<(LNurseSkill, Skill)>(&mut p3.get()?)?
        .grouped_by(&nurses_records)
        .into_iter()
        .zip(nurses)
        .map(SkilledNurse::from)
        .collect();

    Ok(Json(PaginatedResponse::new(res, &q2).total(total as u32)))
}

#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200, body = SkilledNurse),
        (status = 404, body = JsonError)
    ),
    tag = "nurses"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let p2 = pool.clone();

    let nurse: Nurse = macros::get!(nurses, pool, *id, users, addresses);

    let skills: Vec<Skill> = LNurseSkill::belonging_to(&nurse.nurse)
        .inner_join(skills::table)
        .select(skills::all_columns)
        .load(&mut p2.get()?)?;

    let res = SkilledNurse { nurse, skills };

    Ok(Json(res))
}

#[utoipa::path(
    path = "/nurses",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "nurses"
)]
#[post("")]
async fn post(new_record: web::Json<NewNurse>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    web::block(move || {
        insert_into(nurses::table)
            .values(&new_record.0)
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "nurses"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_record: web::Json<UpdateNurse>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(nurses::table)
            .set(&update_record.0)
            .filter(nurses::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "nurses"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    macros::delete!(nurses, pool, *id);

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of availabilities from the given nurse", body = PaginatedAvailabilities),
    ),
    tag = "nurses"
)]
#[get("/{id}/availabilities")]
async fn availabilities(
    query: web::Query<PaginationParam>,
    id: web::Path<i64>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let res: Vec<Availability> = schema::availabilities::table
        .filter(schema::availabilities::id_nurse.eq(*id))
        .limit(query.limit().into())
        .offset(query.offset().into())
        .load(&mut pool.get()?)?;

    let total: i64 = schema::availabilities::table
        .filter(schema::availabilities::id_nurse.eq(*id))
        .count()
        .get_result(&mut pool.get()?)?;

    Ok(Json(
        PaginatedResponse::new(res, &query).total(total as u32),
    ))
}
