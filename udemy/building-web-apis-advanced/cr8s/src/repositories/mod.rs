pub mod authorized_users;
pub mod crates;
pub mod roles;
pub mod rustaceans;
pub mod userroles;
pub mod users;

use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("cr8s")]
pub struct DatabaseConnection(PgPool);
