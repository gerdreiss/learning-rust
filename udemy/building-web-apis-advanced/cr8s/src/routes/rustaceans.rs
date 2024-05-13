use crate::{
    models::rustaceans::RustaceanData, //
    repositories::rustaceans::RustaceanRepository,
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

#[rocket::get("/rustaceans?<offset>&<limit>", rank = 2)]
pub async fn get_rustaceans_paginated(
    mut database: Connection<DatabaseConnection>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::get_paginated(
        &mut database,
        offset.unwrap_or_default(),
        limit.unwrap_or_else(|| i64::MAX),
    )
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

#[rocket::post("/rustaceans", format = "json", data = "<data>")]
pub async fn create_rustacean(
    mut database: Connection<DatabaseConnection>,
    data: Json<RustaceanData>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut database, data.into_inner())
        .await
        .map(|new_rustacean| Custom(Status::Created, json!(new_rustacean)))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<data>")]
pub async fn update_rustacean(
    mut database: Connection<DatabaseConnection>,
    id: i32,
    data: Json<RustaceanData>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut database, id, data.into_inner())
        .await
        .map(|updated_rustacean| json!(updated_rustacean))
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
