#[macro_use]
extern crate rocket;

mod auth;

use auth::BasicAuth;
use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id":1, "name": "John Doe"},{"id":2, "name": "Jane Doe"}])
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john.doe@mail.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe Jr.", "email": "john.doe.jr@mail.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john.doe@mail.com"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: u32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": 404, "message": "Not Found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": 401, "message": "Unauthorized"})
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
    catchers![not_found, unauthorized]
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes())
        .register("/", catchers())
        .launch()
        .await;
}
