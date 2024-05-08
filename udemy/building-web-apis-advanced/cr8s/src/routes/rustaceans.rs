use crate::{
    models::rustaceans::RustaceanData, repositories::rustaceans::RustaceanRepository,
    DatabaseConnection,
};
use diesel::result::Error;
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut database: Connection<DatabaseConnection>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::get_all(&mut database)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(
    mut database: Connection<DatabaseConnection>,
    id: i32,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::get(&mut database, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|error| match error {
            Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
            _ => Custom(Status::InternalServerError, json!(error.to_string())),
        })
}

#[rocket::post("/rustaceans", format = "json", data = "<rustacean>")]
pub async fn create_rustacean(
    mut database: Connection<DatabaseConnection>,
    rustacean: Json<RustaceanData>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut database, rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut database: Connection<DatabaseConnection>,
    id: i32,
    rustacean: Json<RustaceanData>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut database, id, rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|error| match error {
            Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
            _ => Custom(Status::InternalServerError, json!(error.to_string())),
        })
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut database: Connection<DatabaseConnection>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut database, id)
        .await
        .map(|_| NoContent)
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}
