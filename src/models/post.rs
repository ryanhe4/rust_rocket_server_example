use crate::schema::posts;

use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub type_: i32,
    pub content: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub fk_user_id: i32,
}
