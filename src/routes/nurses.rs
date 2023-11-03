use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::{has_any_role, has_roles};
use diesel::{
    connection::DefaultLoadingMode, insert_into, BelongingToDsl, ExpressionMethods, GroupedBy,
    QueryDsl, RunQueryDsl, SelectableHelper,
};
use macros::{list, total};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    schema::{
        self, addresses, l_visits_nurses, mission_types, missions, nurses, patients, skills, users,
        visits,
    },
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete, availabilities, ical),
    components(schemas(
        Nurse,
        SkilledNurse,
        NurseRecord,
        UpdateNurse,
        NewNurseRecord,
        NewNurse,
        User,
        NewUser,
        NewAddress,
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
        .service(ical)
}

#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of nurses", body = PaginatedNurses),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager"])
    )
)]
#[get("")]
#[has_roles("Role::Manager", type = "Role")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
    _: Auth,
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
    tag = "nurses",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("/{id}")]
#[has_any_role["Role::Manager", "Role::Nurse", type = "Role"]]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let p2 = pool.clone();

    if auth.role == Role::Nurse && auth.id != *id {
        return Err(actix_web::error::ErrorForbidden("").into());
    }

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
    tag = "nurses",
    security(
        ("token" = ["manager"])
    )
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_record: web::Json<NewNurse>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    pool.get()?.build_transaction().run(|conn| {
        let NewNurse {
            nurse,
            user,
            address,
        } = new_record.0;

        let id_address: i64 = insert_into(addresses::table)
            .values(&address)
            .returning(addresses::id)
            .get_result(conn)?;

        let id_user: i64 = insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result(conn)?;

        insert_into(nurses::table)
            .values(NewNurseRecord {
                id_user,
                id_address,
                ..nurse
            })
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[put("/{id}")]
#[has_any_role["Role::Manager", "Role::Nurse", type = "Role"]]
async fn put(
    id: web::Path<i64>,
    update_record: web::Json<UpdateNurse>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    if auth.role == Role::Nurse && auth.id != *id {
        return Err(actix_web::error::ErrorForbidden("").into());
    }

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
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(nurses, pool, *id);

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of availabilities from the given nurse", body = PaginatedAvailabilities),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("/{id}/availabilities")]
#[has_any_role["Role::Manager", "Role::Nurse", type = "Role"]]
async fn availabilities(
    query: web::Query<PaginationParam>,
    id: web::Path<i64>,
    pool: web::Data<DbPool>,
    _: Auth,
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

    let cal: Result<Calendar> = visits.map(|visit| Ok(Event::from(visit?))).collect();

    cal.map(|mut c| {
        c.name(&format!(
            "Planning de {} {}",
            nurse.fname,
            nurse.lname.to_uppercase()
        ));

        c.to_string()
    })
}
