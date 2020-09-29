use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub content: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
