use crate::schema::users_roles;
use diesel::prelude::*;
use serde::*;

#[derive(Queryable, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = users_roles)]
pub struct UserRoleData {
    pub user_id: i32,
    pub role_id: i32,
}
