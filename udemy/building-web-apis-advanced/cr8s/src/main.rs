use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("cr8s")]
struct DatabaseConnection(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
fn get_rustaceans(database: Connection<DatabaseConnection>) {
    // TODO
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DatabaseConnection::init())
        .launch()
        .await;
}
