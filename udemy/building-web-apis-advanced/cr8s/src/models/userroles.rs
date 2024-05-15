use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::models::roles::Role;
use crate::models::users::User;
use crate::schema::users_roles;

#[derive(Queryable, Serialize, Debug, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
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
