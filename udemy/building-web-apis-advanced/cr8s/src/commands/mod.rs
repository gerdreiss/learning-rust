use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;

use crate::models::roles::RoleData;
use crate::models::userroles::UserRoleData;
use crate::models::users::UserData;
use crate::repositories::roles::RoleRepository;
use crate::repositories::userroles::UserRoleRepository;
use crate::repositories::users::UserRepository;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Error connecting to database")
}

pub async fn create_role(code: String, name: String) {
    let mut c = load_db_connection().await;
    let role = RoleRepository::create(&mut c, RoleData { code, name })
        .await
        .expect("Error creating role");
    println!("created role {:?}", role);
}

pub async fn list_roles() {
    let mut c = load_db_connection().await;
    let roles = RoleRepository::get_all(&mut c)
        .await
        .expect("Error listing roles");
    println!("{:?}", roles);
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let user = UserRepository::create(&mut c, UserData { username, password })
        .await
        .expect("Error creating user");

    let user_roles = RoleRepository::get_all_by_codes(&mut c, role_codes)
        .await
        .expect("Error getting roles")
        .into_iter()
        .map(|role| UserRoleData {
            user_id: user.id,
            role_id: role.id,
        })
        .collect();

    let _user_roles = UserRoleRepository::create_multiple(&mut c, user_roles)
        .await
        .expect("Error creating user roles");

    let role_ids = UserRoleRepository::get_all_by_user(&mut c, &user)
        .await
        .expect("Error getting user roles")
        .into_iter()
        .map(|user_role| user_role.role_id)
        .collect();

    let roles = RoleRepository::get_all_by_ids(&mut c, role_ids)
        .await
        .expect("Error getting roles by user role ids");

    println!("created user {:?} with roles {:?}", user, roles);
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
