mod users;
use rocket_sync_db_pools::{database, diesel};

#[database("tablog")]
pub struct Conn(diesel::MysqlConnection);
