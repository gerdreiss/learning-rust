use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::crates;

#[derive(Queryable, Serialize)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crates)]
pub struct CrateData {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
