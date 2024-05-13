use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::roles;

#[derive(Queryable, Serialize)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = roles)]
pub struct RoleData {
    pub code: String,
    pub name: String,
}
