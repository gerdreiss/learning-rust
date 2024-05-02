#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use diesel::result::Error;
use models::*;
use repositories::RustaceanRepository;
use rocket::{
    fairing::AdHoc,
    http::Status,
    response::status::{self, Custom},
    serde::json::{json, Json, Value},
    Build, Rocket,
};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DatabaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DatabaseConnection) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 1000)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DatabaseConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|error| match error {
                Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
                _ => Custom(Status::InternalServerError, json!(error.to_string())),
            })
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DatabaseConnection,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DatabaseConnection,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|error| match error {
                Error::NotFound => Custom(Status::NotFound, json!(error.to_string())),
                _ => Custom(Status::InternalServerError, json!(error.to_string())),
            })
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DatabaseConnection,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": 404, "message": "Not Found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": 401, "message": "Unauthorized"})
}

#[catch(422)]
fn unprocessable_content() -> Value {
    json!({"status": 422, "message": "Unprocessable content"})
}

fn routes() -> Vec<rocket::Route> {
    routes![
        get_rustaceans,
        get_rustacean,
        create_rustacean,
        update_rustacean,
        delete_rustacean
    ]
}

fn catchers() -> Vec<rocket::Catcher> {
    catchers![not_found, unauthorized, unprocessable_content]
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    DatabaseConnection::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS)
                .expect("Failed to run migrations");
        })
        .await;

    rocket
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes())
        .register("/", catchers())
        .attach(DatabaseConnection::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_db_migrations))
        .launch()
        .await;
}
