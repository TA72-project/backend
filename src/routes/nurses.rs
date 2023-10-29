use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{
    connection::DefaultLoadingMode, insert_into, BelongingToDsl, ExpressionMethods, GroupedBy,
    QueryDsl, RunQueryDsl, SelectableHelper,
};
use macros::{list, total};

use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{
        addresses, l_visits_nurses, mission_types, missions, nurses, patients, skills, users,
        visits,
    },
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete, ical),
    components(schemas(
        Nurse,
        SkilledNurse,
        NurseRecord,
        UpdateNurse,
        NewNurse,
        User,
        crate::pagination::PaginatedNurses,
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
        .service(ical)
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
    responses(
        (status = 200, body = String, description = "Icalendar data"),
        (status = 404, body = JsonError)
    ),
    tag = "nurses"
)]
#[get("/{id}/ical")]
async fn ical(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    use icalendar::*;

    let nurse: User = users::table
        .inner_join(nurses::table)
        .filter(nurses::id.eq(*id))
        .select(users::all_columns)
        .first(&mut pool.get()?)?;

    let visits = visits::table
        .inner_join(
            missions::table.inner_join(mission_types::table).inner_join(
                patients::table
                    .inner_join(users::table)
                    .inner_join(addresses::table),
            ),
        )
        .inner_join(l_visits_nurses::table)
        .filter(l_visits_nurses::id_nurse.eq(*id))
        .select(Visit::as_select())
        .load_iter::<Visit, DefaultLoadingMode>(&mut pool.get()?)?;

    let mut cal: Calendar = visits.map(|visit| Event::from(visit.unwrap())).collect();

    cal.name(&format!(
        "Planning de {} {}",
        nurse.fname,
        nurse.lname.to_uppercase()
    ));

    Ok(cal.to_string())
}
