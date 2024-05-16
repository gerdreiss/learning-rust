use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::userroles::*;
use crate::models::users::User;
use crate::schema::users_roles;

pub struct UserRoleRepository;

impl UserRoleRepository {
    pub async fn create(
        c: &mut AsyncPgConnection,
        new_user_role: UserRoleData,
    ) -> QueryResult<UserRole> {
        diesel::insert_into(users_roles::table)
            .values(new_user_role)
            .get_result(c)
            .await
    }

    pub async fn create_multiple(
        c: &mut AsyncPgConnection,
        new_user_roles: Vec<UserRoleData>,
    ) -> QueryResult<Vec<UserRole>> {
        diesel::insert_into(users_roles::table)
            .values(&new_user_roles)
            .get_results(c)
            .await
    }

    pub async fn get_all_by_user(
        c: &mut AsyncPgConnection,
        user: &User,
    ) -> QueryResult<Vec<UserRole>> {
        UserRole::belonging_to(&user)
            .select(UserRole::as_select())
            .load(c)
            .await
    }
}
