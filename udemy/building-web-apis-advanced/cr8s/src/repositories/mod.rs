pub mod crates;
pub mod roles;
pub mod rustaceans;
pub mod users;

use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("cr8s")]
pub struct DatabaseConnection(PgPool);
