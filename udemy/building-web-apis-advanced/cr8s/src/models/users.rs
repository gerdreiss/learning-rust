use crate::schema::users;
use diesel::prelude::*;
use serde::*;

#[derive(Queryable, Serialize)]
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
