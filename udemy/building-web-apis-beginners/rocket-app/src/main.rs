#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::*;
use repositories::RustaceanRepository;
use rocket::{
    futures::FutureExt,
    response::status,
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DatabaseConnection(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DatabaseConnection) -> Value {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 1000)
            .expect("Error loading rustaceans from database");
    })
    .map(|rustaceans| json!(rustaceans))
    .await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DatabaseConnection) -> Value {
    db.run(move |c| {
        RustaceanRepository::find(c, id).expect("Error loading rustacean id {id} from database");
    })
    .map(|rustacean| json!(rustacean))
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DatabaseConnection,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .expect("Error saving new rustacean")
    })
    .map(|rustacean| json!(rustacean))
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DatabaseConnection,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .expect("Error updating rustacean with id {id}")
    })
    .map(|updated| json!(updated))
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DatabaseConnection) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id).expect("Error deleting rustacean with id {id}");
        status::NoContent
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

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes())
        .register("/", catchers())
        .attach(DatabaseConnection::fairing())
        .launch()
        .await;
}
