use rocket_db_pools::{diesel::PgPool, Database};

mod models;
mod repositories;
mod routes;
mod schema;

#[derive(Database)]
#[database("cr8s")]
struct DatabaseConnection(PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::rustaceans::get_rustaceans,
                routes::rustaceans::get_rustacean,
                routes::rustaceans::create_rustacean,
                routes::rustaceans::update_rustacean,
                routes::rustaceans::delete_rustacean,
                routes::crates::get_crates,
                routes::crates::get_crate,
                routes::crates::create_crate,
                routes::crates::update_crate,
                routes::crates::delete_crate
            ],
        )
        .attach(DatabaseConnection::init())
        .launch()
        .await;
}
