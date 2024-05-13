use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::users::*;
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(c: &mut AsyncPgConnection, new_user: UserData) -> QueryResult<User> {
        diesel::insert_into(crate::schema::users::table)
            .values(new_user)
            .get_result(c)
            .await
    }

    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<User>> {
        users::table.load(c).await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}
