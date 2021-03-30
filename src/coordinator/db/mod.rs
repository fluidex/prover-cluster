use sqlx;

pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type DBErrType = sqlx::Error;

// TODO: migrate

pub mod models;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"
