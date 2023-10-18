use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

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
pub struct SkillDoc;

pub fn routes() -> Scope {
    web::scope("/skills")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

#[utoipa::path(
    get,
    path = "/skills",
    responses(
        (status = 200, description = "Paginated list of skills", body = PaginatedSkills),
    ),
    params(
        PaginationParam
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

    let skills: Vec<Skill> = web::block(move || {
        skills::table
            .select(skills::all_columns)
            .offset(query.offset().into())
            .limit(query.limit().into())
            .get_results(&mut pool.get().unwrap())
    })
    .await??;

    let total: i64 =
        web::block(move || skills::table.count().get_result(&mut p2.get().unwrap())).await??;

    Ok(Json(
        PaginatedResponse::new(skills, &q2).total(total as u32),
    ))
}

#[utoipa::path(
    get,
    path = "/skills/{id}",
    responses(
        (status = 200, body = Skill),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i64, Path, description = "Skill id")
    ),
    tag = "skills"
)]
#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let skill: Skill =
        web::block(move || skills::table.find(*id).get_result(&mut pool.get().unwrap())).await??;

    Ok(Json(skill))
}

#[utoipa::path(
    post,
    path = "/skills",
    responses(
        (status = 200, body = Skill),
        (status = 400)
    ),
    tag = "skills"
)]
#[post("")]
async fn post(new_skill: web::Json<NewSkill>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let skill: Skill = web::block(move || {
        insert_into(skills::table)
            .values(&new_skill.0)
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(skill))
}

#[utoipa::path(
    put,
    path = "/skills/{id}",
    responses(
        (status = 200, body = Skill),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the skill to update")
    ),
    tag = "skills"
)]
#[put("/{id}")]
async fn put(
    id: web::Path<i64>,
    update_skill: web::Json<UpdateSkill>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let skill: Skill = web::block(move || {
        diesel::update(skills::table)
            .set(&update_skill.0)
            .filter(skills::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(skill))
}

#[utoipa::path(
    delete,
    path = "/skills/{id}",
    responses(
        (status = 200, body = Skill, description = "The deleted skill"),
        (status = 400)
    ),
    params(
        ("id" = i64, Path, description = "Id of the skill to delete")
    ),
    tag = "skills"
)]
#[delete("/{id}")]
async fn delete(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let skill: Skill = web::block(move || {
        diesel::delete(skills::table)
            .filter(skills::id.eq(*id))
            .get_result(&mut pool.get().unwrap())
    })
    .await??;

    Ok(Json(skill))
}
