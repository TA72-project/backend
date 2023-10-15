use actix_web::{
    get, post,
    web::{self, Json},
    Responder, Scope,
};
use diesel::{insert_into, QueryDsl, RunQueryDsl};

use crate::{
    database::DbPool,
    error::Result,
    models::{NewSkill, Skill},
    schema::skills,
};

pub fn routes() -> Scope {
    web::scope("/skills")
        .service(all)
        .service(get)
        .service(post)
}

#[get("")]
async fn all() -> impl Responder {
    "Skills list".to_owned()
}

#[get("/{id}")]
async fn get(id: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let res: Skill =
        web::block(move || skills::table.find(*id).get_result(&mut pool.get().unwrap())).await??;

    Ok(Json(res))
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
