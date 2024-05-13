use crate::{
    models::crates::CrateData, //
    repositories::crates::CrateRepository,
    DatabaseConnection,
};
use diesel::result::Error;
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/crates")]
pub async fn get_crates(
    mut database: Connection<DatabaseConnection>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::get_all(&mut database)
        .await
        .map(|crates| json!(crates))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::get("/crates?<offset>&<limit>", rank = 2)]
pub async fn get_crates_paginated(
    mut database: Connection<DatabaseConnection>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::get_paginated(
        &mut database,
        offset.unwrap_or_default(),
        limit.unwrap_or_else(|| i64::MAX),
    )
    .await
    .map(|crates| json!(crates))
    .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(
    mut database: Connection<DatabaseConnection>,
    id: i32,
) -> Result<Value, Custom<Value>> {
    CrateRepository::get(&mut database, id)
        .await
        .map(|the_crate| json!(the_crate))
        .map_err(|error| match error {
            Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
            _ => Custom(Status::InternalServerError, json!(error.to_string())),
        })
}

#[rocket::post("/crates", format = "json", data = "<data>")]
pub async fn create_crate(
    mut database: Connection<DatabaseConnection>,
    data: Json<CrateData>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut database, data.into_inner())
        .await
        .map(|new_crate| Custom(Status::Created, json!(new_crate)))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::put("/crates/<id>", format = "json", data = "<data>")]
pub async fn update_crate(
    mut database: Connection<DatabaseConnection>,
    id: i32,
    data: Json<CrateData>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut database, id, data.into_inner())
        .await
        .map(|updated_crate| json!(updated_crate))
        .map_err(|error| match error {
            Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
            _ => Custom(Status::InternalServerError, json!(error.to_string())),
        })
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut database: Connection<DatabaseConnection>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut database, id)
        .await
        .map(|_| NoContent)
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}
