use crate::db::Conn;
use crate::schema::social_accounts;
use diesel::prelude::*;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct SocialAccount {
    pub id: i32,
    pub provider: String,
    pub social_id: String,
    pub user_id: Option<i32>,
}

impl SocialAccount {
    pub async fn find_by_social_id(id: String, conn: &Conn) -> QueryResult<SocialAccount> {
        use social_accounts::dsl::{social_accounts as SA, social_id};
        conn.run(move |c| {
            SA.filter(social_accounts::social_id.eq(id.as_str()))
                .filter(social_accounts::provider.eq("google"))
                .get_result::<SocialAccount>(c)
        })
        .await
    }
}
