use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::roles::*;
use crate::schema::roles;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create(c: &mut AsyncPgConnection, new_role: RoleData) -> QueryResult<Role> {
        diesel::insert_into(crate::schema::roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }

    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Role>> {
        roles::table.load(c).await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(c).await
    }
}
