use actix_web::{
    delete,
    error::ErrorForbidden,
    get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::{has_any_role, has_roles};
use diesel::{
    connection::DefaultLoadingMode, insert_into, BelongingToDsl, ExpressionMethods, GroupedBy,
    PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::*,
    pagination::{PaginatedResponse, PaginationParam},
    params::{SearchParam, SortParam},
    schema::{
        self, addresses, l_nurses_skills, l_visits_nurses, mission_types, missions, nurses,
        patients, skills, users, visits, zones,
    },
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        all,
        get,
        me,
        post,
        post_nurse_skill,
        put,
        delete,
        delete_nurse_skill,
        availabilities,
        reports,
        ical
    ),
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
        LVisitNurse,
        crate::pagination::PaginatedLVisitsNurses,
        crate::pagination::PaginatedSkilledNurses,
        crate::pagination::PaginatedAvailabilities,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/nurses")
        .service(all)
        .service(me)
        .service(get)
        .service(post)
        .service(post_nurse_skill)
        .service(put)
        .service(delete)
        .service(delete_nurse_skill)
        .service(availabilities)
        .service(reports)
        .service(ical)
}

#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of nurses", body = PaginatedSkilledNurses),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager"])
    )
)]
#[get("")]
#[has_roles("Role::Manager", type = "Role")]
async fn all(
    pagination: web::Query<PaginationParam>,
    search: web::Query<SearchParam>,
    sort: web::Query<SortParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let pool = &mut pool.get()?;

    // Get nurses
    let nurses: Vec<Nurse> = nurses::table
        .inner_join(users::table)
        .inner_join(addresses::table)
        .filter(users::fname.ilike(search.value()))
        .or_filter(users::lname.ilike(search.value()))
        .or_filter(users::mail.ilike(search.value()))
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(pool)?;

    // Get total of nurses
    let total = nurses::table
        .inner_join(users::table)
        .filter(users::fname.ilike(search.value()))
        .or_filter(users::lname.ilike(search.value()))
        .or_filter(users::mail.ilike(search.value()))
        .count()
        .get_result::<i64>(pool)?;

    // Get database records
    let nurses_records: Vec<_> = nurses.iter().map(|n| n.nurse).collect();

    // Get skills and group by nurse
    let res = LNurseSkill::belonging_to(&nurses_records)
        .inner_join(skills::table)
        .load::<(LNurseSkill, Skill)>(pool)?
        .grouped_by(&nurses_records)
        .into_iter()
        .zip(nurses)
        .map(SkilledNurse::from)
        .collect();

    Ok(Json(
        PaginatedResponse::new(res, &pagination).total(total as u32),
    ))
}

#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200, body = SkilledNurse),
    ),
    tag = "nurses",
    security(
        ("token" = ["nurse"])
    )
)]
#[get("/me")]
#[has_any_role["Role::Nurse", type = "Role"]]
async fn me(pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let p2 = pool.clone();

    let nurse: Nurse = macros::get!(nurses, pool, auth.id, users, addresses);

    let skills: Vec<Skill> = LNurseSkill::belonging_to(&nurse.nurse)
        .inner_join(skills::table)
        .select(skills::all_columns)
        .load(&mut p2.get()?)?;

    let res = SkilledNurse { nurse, skills };

    Ok(Json(res))
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
        return Err(ErrorForbidden("").into());
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

/// Associate nurse & skill
///
/// Associates the given nurse with the given skill.
#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager"])
    )
)]
#[post("/{id_nurse}/skills/{id_skill}")]
#[has_roles("Role::Manager", type = "Role")]
async fn post_nurse_skill(
    ids: web::Path<(i64, i64)>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    let id_center: i64 = nurses::table
        .inner_join(addresses::table.inner_join(zones::table))
        .select(zones::id_center)
        .get_result(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    insert_into(l_nurses_skills::table)
        .values(&NewLNurseSkill {
            id_nurse: ids.0,
            id_skill: ids.1,
        })
        .execute(&mut pool.get()?)?;

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
        return Err(ErrorForbidden("").into());
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

/// Delete nurse
///
/// This will also delete the associated user and address.
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
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, auth: Auth) -> Result<impl Responder> {
    let id_center: i64 = nurses::table
        .inner_join(addresses::table.inner_join(zones::table))
        .select(zones::id_center)
        .get_result(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    pool.get()?.build_transaction().run(|conn| {
        let (id_user, id_address): (i64, i64) = diesel::delete(nurses::table)
            .filter(nurses::id.eq(*id))
            .returning((nurses::id_user, nurses::id_address))
            .get_result(conn)?;

        diesel::delete(users::table)
            .filter(users::id.eq(id_user))
            .execute(conn)?;

        diesel::delete(addresses::table)
            .filter(addresses::id.eq(id_address))
            .execute(conn)?;

        Ok::<(), diesel::result::Error>(())
    })?;

    Ok(Json(()))
}

/// Dissociate nurse & skill
///
/// Dissociates the given nurse with the given skill.
#[utoipa::path(
    context_path = "/nurses",
    responses(
        (status = 200),
        (status = 404, body = JsonError)
    ),
    tag = "nurses",
    security(
        ("token" = ["manager"])
    )
)]
#[delete("/{id_nurse}/skills/{id_skill}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete_nurse_skill(
    ids: web::Path<(i64, i64)>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    let id_center: i64 = nurses::table
        .inner_join(addresses::table.inner_join(zones::table))
        .select(zones::id_center)
        .get_result(&mut pool.get()?)?;

    if id_center != auth.id_center {
        return Err(ErrorForbidden("").into());
    }

    let rows = diesel::delete(l_nurses_skills::table)
        .filter(l_nurses_skills::id_nurse.eq(ids.0))
        .filter(l_nurses_skills::id_skill.eq(ids.1))
        .execute(&mut pool.get()?)?;

    if rows == 0 {
        Err(diesel::result::Error::NotFound.into())
    } else {
        Ok(Json(()))
    }
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
    auth: Auth,
) -> Result<impl Responder> {
    if auth.role == Role::Nurse && auth.id != *id {
        return Err(ErrorForbidden("A nurse can only access its own availabilities").into());
    }

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

/// Nurse's reports
///
/// Get the reports from a nurse. A manager can access every nurse. A nurse can only access itself.
/// Reports that are either null or empty are not returned.
#[utoipa::path(
    context_path = "/nurses",
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of reports", body = PaginatedLVisitsNurses),
    ),
    tag = "nurses",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("/{id}/reports")]
#[has_any_role["Role::Manager", "Role::Nurse", type = "Role"]]
async fn reports(
    query: web::Query<PaginationParam>,
    id: web::Path<i64>,
    pool: web::Data<DbPool>,
    auth: Auth,
) -> Result<impl Responder> {
    if auth.role == Role::Nurse && auth.id != *id {
        return Err(ErrorForbidden("A nurse can only access its own reports").into());
    }

    let res: Vec<LVisitNurse> = schema::l_visits_nurses::table
        .filter(schema::l_visits_nurses::id_nurse.eq(*id))
        .filter(schema::l_visits_nurses::report.is_not_null())
        .filter(schema::l_visits_nurses::report.ne(""))
        .limit(query.limit().into())
        .offset(query.offset().into())
        .load(&mut pool.get()?)?;

    let total: i64 = schema::l_visits_nurses::table
        .filter(schema::l_visits_nurses::id_nurse.eq(*id))
        .filter(schema::l_visits_nurses::report.is_not_null())
        .filter(schema::l_visits_nurses::report.ne(""))
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
