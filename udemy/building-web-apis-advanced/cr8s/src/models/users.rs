use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::users;

#[derive(Queryable, Serialize, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UserData {
    pub username: String,
    pub password: String,
}
