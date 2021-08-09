use crate::schema::likes;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(post_id, user_id)]
pub struct Like {
    pub post_id: i32,
    pub user_id: i32,
}
