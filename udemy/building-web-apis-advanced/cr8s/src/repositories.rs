use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{models::persistence::*, schema::*};

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

pub struct CrateRepository;

impl CrateRepository {
    pub async fn get(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).first(c).await
    }

    pub async fn get_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Crate>> {
        crates::table.load(c).await
    }

    pub async fn get_paginated(
        c: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Crate>> {
        crates::table.offset(offset).limit(limit).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, data: CrateData) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(&data)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, data: CrateData) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set(data)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c).await
    }
}
