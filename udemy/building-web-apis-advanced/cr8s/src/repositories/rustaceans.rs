use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::models::rustaceans::*;
use crate::schema::rustaceans;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn get(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).first(c).await
    }

    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.load(c).await
    }

    pub async fn get_paginated(
        c: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.offset(offset).limit(limit).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, data: RustaceanData) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(&data)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        id: i32,
        data: RustaceanData,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set(data)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}
