use repositories::RustaceanRepository;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("cr8s")]
struct DatabaseConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(
    mut database: Connection<DatabaseConnection>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::get_all(&mut database)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DatabaseConnection::init())
        .launch()
        .await;
}
