#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
extern crate reqwest;

mod app;
mod db;
mod models;
mod route;
mod schema;
mod util;

use std::env;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    app::rocket()
        .launch()
        .await
        .expect("error raised on Rocket Main Application");
}
