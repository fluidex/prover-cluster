pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
pub type DBErrType = sqlx::Error;

pub mod models;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"
