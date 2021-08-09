use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::response::status::Conflict;
use rocket::response::status::Custom;
use rocket::response::status::Unauthorized;
use rocket::response::Responder;
use rocket::serde::json::{serde_json, Value};

#[derive(Responder)]
pub enum Error {
    E401(Unauthorized<Value>),
    E409(Conflict<Value>),
    E500(Custom<Value>),
}

pub enum ErrorCode {
    E401,
    E409,
    E500,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub code: i32,
    pub error: String,
    pub msg: String,
}

pub fn to_error(code: ErrorCode, msg: Message) -> Error {
    let json = serde_json::to_value(msg).unwrap();
    match code {
        ErrorCode::E401 => Error::E401(Unauthorized(Some(json))),
        ErrorCode::E409 => Error::E409(Conflict(Some(json))),
        ErrorCode::E500 => Error::E500(Custom(Status::InternalServerError, json)),
    }
}
