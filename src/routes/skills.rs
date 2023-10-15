use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    database::DbPool,
    error::Result,
    models::{NewSkill, Skill, UpdateSkill},
    pagination::{PaginatedResponse, PaginationParam},
    schema::skills,
};

pub fn routes() -> Scope {
    web::scope("/skills")
        .service(all)
        .service(get)
        .service(post)
        .service(put)
        .service(delete)
}

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

#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let skill: Skill =
        web::block(move || skills::table.find(*id).get_result(&mut pool.get().unwrap())).await??;

    Ok(Json(skill))
}

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
