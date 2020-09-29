use super::models::{NewPost, Post};
use super::schema::posts::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub async fn get_posts(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_posts(db))
        .await
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_post_by_id(
    db: web::Data<Pool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_post_by_id(db, post_id.into_inner()))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_post(
    db: web::Data<Pool>,
    item: web::Json<InputPost>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_post(db, item))
        .await
        .map(|post| HttpResponse::Created().json(post))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_post(
    db: web::Data<Pool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_post(db, post_id.into_inner()))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_posts(pool: web::Data<Pool>) -> Result<Vec<Post>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = posts.load::<Post>(&conn)?;
    Ok(items)
}

fn db_get_post_by_id(pool: web::Data<Pool>, post_id: i32) -> Result<Post, diesel::result::Error> {
    let conn = pool.get().unwrap();
    posts.find(post_id).get_result::<Post>(&conn)
}

fn add_single_post(
    db: web::Data<Pool>,
    item: web::Json<InputPost>,
) -> Result<Post, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_post = NewPost {
        title: &item.title,
        author: &item.author,
        content: &item.content,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(posts).values(&new_post).get_result(&conn)?;
    Ok(res)
}

fn delete_single_post(db: web::Data<Pool>, post_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(posts.find(post_id)).execute(&conn)?;
    Ok(count)
}
