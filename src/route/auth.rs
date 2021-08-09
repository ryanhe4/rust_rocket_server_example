//rocket
use rocket::fairing::AdHoc;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
//DB
use crate::db::Conn;
use crate::models::{SocialAccount, User};

//Util
use crate::util::error::{Error as RouteError, *};
use crate::util::google::get_google_profile;

#[derive(Deserialize)]
struct GoogleAccessTokenSchema {
    access_token: String,
}
#[post("/google/check", format = "application/json", data = "<request>")]
async fn check(db: Conn, request: Json<GoogleAccessTokenSchema>) -> Result<Value, RouteError> {
    let access_token = &request.access_token;
    let profile = get_google_profile(access_token.to_string()).await;
    let profile = match profile {
        Ok(p) => p,
        Err(_) => {
            let result = Message {
                code: 401,
                error: "UNAUTHENTICATED".to_owned(),
                msg: "Request had invalid authentication credentials".to_owned(),
            };
            return Err(to_error(ErrorCode::E401, result));
        }
    };
    if let Err(e) = SocialAccount::find_by_social_id(profile.social_id, &db).await {
        return match e {
            diesel::NotFound => Ok(json!({
              "exists": false
            })),
            _ => {
                let result = Message {
                    code: 500,
                    error: "REQUESTFAILED".to_owned(),
                    msg: "Fail".to_owned(),
                };
                Err(to_error(ErrorCode::E500, result))
            }
        };
    }
    Ok(json!({
      "exists": true
    }))
}

#[post("/google/signin", format = "application/json", data = "<request>")]
async fn sign_in(db: Conn, request: Json<GoogleAccessTokenSchema>) -> Result<Value, RouteError> {
    let access_token = &request.access_token;
    let profile = get_google_profile(access_token.to_string()).await;
    let profile = match profile {
        Ok(p) => p,
        Err(_) => {
            let result = Message {
                code: 401,
                error: "UNAUTHENTICATED".to_owned(),
                msg: "Request had invalid authentication credentials".to_owned(),
            };
            return Err(to_error(ErrorCode::E401, result));
        }
    };
    let exist = match SocialAccount::find_by_social_id(profile.social_id, &db).await {
        Ok(exist) => exist,
        Err(e) => {
            return match e {
                diesel::NotFound => {
                    let result = Message {
                        code: 401,
                        error: "UserNotFoundError".to_owned(),
                        msg: "User is not registered".to_owned(),
                    };
                    Err(to_error(ErrorCode::E401, result))
                }
                _ => {
                    let result = Message {
                        code: 500,
                        error: "REQUESTFAILED".to_owned(),
                        msg: "Fail".to_owned(),
                    };
                    Err(to_error(ErrorCode::E500, result))
                }
            }
        }
    };

    let user = match User::find_one(exist.user_id.unwrap(), &db).await {
        Ok(user) => user,
        Err(e) => {
            return match e {
                diesel::NotFound => {
                    let result = Message {
                        code: 500,
                        error: "UserNotFoundError".to_owned(),
                        msg: "Google login succeed but user not found".to_owned(),
                    };
                    Err(to_error(ErrorCode::E500, result))
                }
                _ => {
                    let result = Message {
                        code: 500,
                        error: "REQUESTFAILED".to_owned(),
                        msg: "Fail".to_owned(),
                    };
                    Err(to_error(ErrorCode::E500, result))
                }
            }
        }
    };
    user.generate_token();

    Ok(json!({
      "exists": true
    }))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Stage", |rocket| async {
        rocket
            .attach(Conn::fairing())
            .mount("/api/auth", routes![sign_in, check])
    })
}
