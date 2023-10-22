use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use macros::{list, total};

use crate::{
    database::DbPool,
    error::{JsonError, Result},
    models::{NewSkill, Skill, UpdateSkill},
    pagination::{PaginatedResponse, PaginationParam},
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
    params(PaginationParam),
    responses(
        (status = 200, description = "Paginated list of skills", body = PaginatedSkills),
    ),
    tag = "skills"
)]
#[get("")]
async fn all(
    query: web::Query<PaginationParam>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let q2 = query.clone();
    let p2 = pool.clone();

    let skills: Vec<Skill> = list!(skills, pool, query);

    let total = total!(skills, p2);

    Ok(Json(PaginatedResponse::new(skills, &q2).total(total)))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200, body = Skill),
        (status = 404, body = JsonError),
    ),
    tag = "skills"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let skill: Skill = macros::get!(skills, pool, *id);

    Ok(Json(skill))
}

#[utoipa::path(
    context_path = "/skills",
    responses(
        (status = 200),
        (status = 400, body = JsonError),
    ),
    tag = "skills"
)]
#[post("")]
async fn post(new_skill: web::Json<NewSkill>, pool: web::Data<DbPool>) -> Result<impl Responder> {
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
    tag = "skills"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_skill: web::Json<UpdateSkill>,
    pool: web::Data<DbPool>,
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
    tag = "skills"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    macros::delete!(skills, pool, *id);

    Ok(Json(()))
}
