use crate::db::Conn;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub display_name: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub is_certified: i8,
}

impl User {
    pub async fn find_one(_id: i32, conn: &Conn) -> QueryResult<User> {
        use users::dsl::{id, users as USER};
        conn.run(move |c| USER.filter(users::id.eq(_id)).get_result::<User>(c))
            .await
    }
    pub fn generate_token(&self) {
        println!("Token is generated");
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub photo_url: String,
    pub is_certified: i8,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
}
