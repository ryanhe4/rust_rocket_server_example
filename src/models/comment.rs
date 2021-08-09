use chrono::NaiveDateTime;

use crate::schema::comments;

#[derive(Queryable, Debug, Identifiable)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub reply_to: Option<i32>,
    pub likes: i32,
    pub level: i32,
    pub has_replies: i8, /* TODO: unknown type Tinyint */
    pub deleted: i8,     /* TODO: unknown type Tinyint */
    pub fk_user_id: i32,
    pub fk_post_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
