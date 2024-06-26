use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::rustaceans;

#[derive(Queryable, Serialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct RustaceanData {
    pub name: String,
    pub email: String,
}
