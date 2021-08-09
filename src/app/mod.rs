use dotenv::dotenv;
use rocket::response::Debug;
use rocket::{Build, Config, Rocket};

use crate::route::auth;

// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();
    let rocket = rocket::build();

    rocket.mount("/", routes!(index)).attach(auth::stage())
}
