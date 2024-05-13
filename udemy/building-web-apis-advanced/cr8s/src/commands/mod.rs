use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;

use crate::models::users::UserData;
use crate::repositories::users::UserRepository;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Error connecting to database")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let new_user = UserData { username, password };
    let user = UserRepository::create(&mut c, new_user)
        .await
        .expect("Error creating user");
    println!("created {:?}", user);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
    let users = UserRepository::get_all(&mut c)
        .await
        .expect("Error listing users");
    println!("{:?}", users);
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    let deleted = UserRepository::delete(&mut c, id)
        .await
        .expect("Error deleting user");
    println!("deleted {:?}", deleted);
}
