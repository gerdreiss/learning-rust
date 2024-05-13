use rocket_db_pools::Database;

extern crate cr8s;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                cr8s::routes::rustaceans::get_rustaceans,
                cr8s::routes::rustaceans::get_rustaceans_paginated,
                cr8s::routes::rustaceans::get_rustacean,
                cr8s::routes::rustaceans::create_rustacean,
                cr8s::routes::rustaceans::update_rustacean,
                cr8s::routes::rustaceans::delete_rustacean,
                cr8s::routes::crates::get_crates,
                cr8s::routes::crates::get_crates_paginated,
                cr8s::routes::crates::get_crate,
                cr8s::routes::crates::create_crate,
                cr8s::routes::crates::update_crate,
                cr8s::routes::crates::delete_crate
            ],
        )
        .attach(cr8s::repositories::DatabaseConnection::init())
        .launch()
        .await;
}
