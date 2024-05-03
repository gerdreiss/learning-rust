pub mod persistence {
    use crate::schema::*;
    use diesel::prelude::*;

    #[derive(Queryable)]
    pub struct Rustacean {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub created_at: chrono::NaiveDateTime,
    }

    #[derive(Insertable, AsChangeset)]
    #[diesel(table_name = rustaceans)]
    pub struct RustaceanData {
        pub name: String,
        pub email: String,
    }

    #[derive(Queryable)]
    pub struct Crate {
        pub id: i32,
        pub rustacean_id: i32,
        pub code: String,
        pub name: String,
        pub version: String,
        pub description: Option<String>,
        pub created_at: chrono::NaiveDateTime,
    }

    #[derive(Insertable, AsChangeset)]
    #[table_name = "crates"]
    pub struct CrateData {
        pub rustacean_id: i32,
        pub code: String,
        pub name: String,
        pub version: String,
        pub description: Option<String>,
    }
}
