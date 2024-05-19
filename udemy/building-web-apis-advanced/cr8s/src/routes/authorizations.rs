use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::generation_session_id;
use crate::models::users::UserData;
use crate::repositories::users::UserRepository;
use crate::repositories::DatabaseConnection;
use crate::verify_password;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut database: Connection<DatabaseConnection>,
    credentials: Json<UserData>,
) -> Result<Custom<Value>, Custom<Value>> {
    UserRepository::get_by_username(&mut database, &credentials.username)
        .await
        .map(|user| {
            if verify_password(&credentials.password, &user.password).is_ok() {
                Custom(Status::Ok, json!(generation_session_id()))
            } else {
                Custom(Status::Unauthorized, json!("Invalid credentials"))
            }
        })
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}
