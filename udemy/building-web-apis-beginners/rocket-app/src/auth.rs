use base64::prelude::*;
use lazy_regex::regex_captures;
use rocket::request::{FromRequest, Outcome};

#[derive(Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        regex_captures!("Basic (.*)", header)
            .and_then(|(_, token)| BasicAuth::from_base64_encoded(token))
    }
    fn from_base64_encoded(encoded: &str) -> Option<BasicAuth> {
        let decoded_chars = BASE64_STANDARD.decode(encoded).ok()?;
        let decoded_string = String::from_utf8(decoded_chars).ok()?;
        regex_captures!("(.*):(.*)", &decoded_string) //
            .map(|(_, username, password)| BasicAuth {
                username: username.to_string(),
                password: password.to_string(),
            })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("Authorization")
            .and_then(|header| BasicAuth::from_authorization_header(header))
            .map(Outcome::Success)
            .unwrap_or(Outcome::Error((rocket::http::Status::Unauthorized, ())))
    }
}
