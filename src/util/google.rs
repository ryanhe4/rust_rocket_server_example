use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use rocket::serde::json::Value;

pub struct Profile {
    pub social_id: String,
    pub email: String,
    pub photo: String,
    pub display_name: String,
}

pub async fn get_google_profile(mut access_token: String) -> Result<Profile, reqwest::Error> {
    let auth = format!("Bearer {}", access_token.as_str());
    let params = [
        ("personFields", "names,emailAddresses,photos"),
        ("access_token", access_token.as_str()),
    ];
    let client = reqwest::Client::new();
    let res = client
        .get("https://people.googleapis.com/v1/people/me")
        .query(&params)
        .header(AUTHORIZATION, auth)
        .send()
        .await
        .unwrap();

    match res.error_for_status() {
        Ok(res) => {
            let data = res.json::<serde_json::Value>().await.unwrap();
            let profile = Profile {
                social_id: data["resourceName"]
                    .to_string()
                    .replace("people/", "")
                    .replace("\"", ""),
                email: data["emailAddresses"][0]["value"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                photo: data["photos"][0]["url"].as_str().unwrap().to_owned(),
                display_name: data["names"][0]["displayName"].as_str().unwrap().to_owned(),
            };
            Ok(profile)
        }
        Err(err) => Err(err),
    }
}
