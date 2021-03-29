use sqlx;

pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type DBErrType = sqlx::Error;
