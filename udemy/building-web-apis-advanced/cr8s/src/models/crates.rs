use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
#[table_name = "crates"]
pub struct CrateData {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
