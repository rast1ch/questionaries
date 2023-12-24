use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Debug, Database)]
#[database("postgres_db")]
pub struct DbConn(pub sqlx::PgPool);
