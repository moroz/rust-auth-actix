use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub type DbPool = web::Data<Pool>;
pub type ResponseResult = Result<HttpResponse, Error>;
pub type DbQueryResult<T> = Result<T, diesel::result::Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_users(db: DbPool) -> ResponseResult {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: DbPool) -> DbQueryResult<Vec<User>> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub async fn get_user_by_id(db: DbPool, user_id: web::Path<i32>) -> ResponseResult {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_user_by_id(pool: DbPool, user_id: i32) -> DbQueryResult<User> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

pub async fn add_user(db: DbPool, item: web::Json<InputUser>) -> ResponseResult {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_user(pool: DbPool, item: web::Json<InputUser>) -> DbQueryResult<User> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}
