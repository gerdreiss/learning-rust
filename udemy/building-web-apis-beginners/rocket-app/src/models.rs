pub mod persistence {
    use crate::schema::rustaceans;
    use diesel::prelude::*;

    #[derive(Queryable)]
    pub struct Rustacean {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub created_at: String, // TODO: use chrono crate
    }

    #[derive(Insertable)]
    #[diesel(table_name = rustaceans)]
    pub struct RustaceanData {
        pub name: String,
        pub email: String,
    }
}

pub mod api {
    use rocket::serde::Deserialize;
    use rocket::serde::Serialize;

    #[derive(Deserialize, Serialize)]
    pub struct Rustacean {
        pub id: i32,
        pub name: String,
        pub email: String,
    }
}
