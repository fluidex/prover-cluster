use sqlx;

pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type DBErrType = sqlx::Error;

// TODO: migrate

// TODO: sqlxverf

pub mod models;
pub mod sqlxextend;
