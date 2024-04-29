use diesel::prelude::*;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::schema::rustaceans;

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String, // TODO: use chrono crate
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
