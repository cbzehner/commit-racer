use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("postgres")]
pub(crate) struct DB(sqlx::PgPool);
