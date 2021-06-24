use prover_cluster::coordinator::db::{ConnectionType, MIGRATOR};
use sqlx::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url: String = std::env::var("DB_URL")
        .unwrap_or_else(|_| "postgres://coordinator:coordinator_AA9944@127.0.0.1/prover_cluster".to_string())
        .parse::<String>()
        .expect("parse DB_URL");

    let mut db_conn = ConnectionType::connect(&db_url).await?;
    MIGRATOR.run(&mut db_conn).await?;

    Ok(())
}
