use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use actix_web_grants::proc_macro::{has_any_role, has_roles};
use diesel::{insert_into, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    auth::{Auth, Role},
    database::DbPool,
    error::{JsonError, Result},
    models::{NewSkill, Skill, UpdateSkill},
    pagination::{PaginatedResponse, PaginationParam},
    params::{SearchParam, SortParam},
    schema::skills,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(all, get, post, put, delete),
    components(schemas(
        Skill,
        UpdateSkill,
        NewSkill,
        crate::pagination::PaginatedSkills,
        JsonError
    ))
)]
pub struct Doc;

pub fn routes() -> Scope {
    web::scope("/skills")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    context_path = "/skills",
    params(PaginationParam, SearchParam, SortParam),
    responses(
        (status = 200, description = "Paginated list of skills", body = PaginatedSkills),
    ),
    tag = "skills",
    security(
        ("token" = ["manager", "nurse"])
    )
)]
#[get("")]
#[has_any_role("Role::Manager", "Role::Nurse", type = "Role")]
async fn all(
    pagination: web::Query<PaginationParam>,
    search: web::Query<SearchParam>,
    sort: web::Query<SortParam>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    let skills: Vec<Skill> = skills::table
        .filter(skills::name.ilike(search.value()))
        .order(sort.raw_sql())
        .offset(pagination.offset().into())
        .limit(pagination.limit().into())
        .load(&mut pool.get()?)?;

    let total = skills::table
        .filter(skills::name.ilike(search.value()))
        .count()
        .get_result::<i64>(&mut pool.get()?)? as u32;

    Ok(Json(
        PaginatedResponse::new(skills, &pagination).total(total),
    ))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200, body = Skill),
        (status = 404, body = JsonError),
    ),
    tag = "skills",
    security(
        ("token" = ["manager"])
    )
)]
#[get("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    let skill: Skill = macros::get!(skills, pool, *id);

    Ok(Json(skill))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "skills",
    security(
        ("token" = ["manager"])
    )
)]
#[post("")]
#[has_roles("Role::Manager", type = "Role")]
async fn post(
    new_skill: web::Json<NewSkill>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        insert_into(skills::table)
            .values(&new_skill.0)
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
        (status = 404, body = JsonError),
    ),
    tag = "skills",
    security(
        ("token" = ["manager"])
    )
)]
#[put("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn put(
    id: web::Path<i64>,
    update_skill: web::Json<UpdateSkill>,
    pool: web::Data<DbPool>,
    _: Auth,
) -> Result<impl Responder> {
    web::block(move || {
        diesel::update(skills::table)
            .set(&update_skill.0)
            .filter(skills::id.eq(*id))
            .execute(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(()))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200),
        (status = 404, body = JsonError),
    ),
    tag = "skills",
    security(
        ("token" = ["manager"])
    )
)]
#[delete("/{id}")]
#[has_roles("Role::Manager", type = "Role")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>, _: Auth) -> Result<impl Responder> {
    macros::delete!(skills, pool, *id);

    Ok(Json(()))
}
