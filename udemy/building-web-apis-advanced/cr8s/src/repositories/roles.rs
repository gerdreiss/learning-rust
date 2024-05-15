use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::roles::*;
use crate::schema::roles;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create(c: &mut AsyncPgConnection, new_role: RoleData) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }

    pub async fn get_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Role>> {
        roles::table.load(c).await
    }

    pub async fn get_all_by_ids(
        c: &mut AsyncPgConnection,
        ids: Vec<i32>,
    ) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results(c)
            .await
    }

    pub async fn get_all_by_codes(
        c: &mut AsyncPgConnection,
        codes: Vec<String>,
    ) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::code.eq_any(codes))
            .get_results(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(c).await
    }
}
