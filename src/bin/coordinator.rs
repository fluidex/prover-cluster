use prover_cluster::coordinator::{config, Coordinator};
use prover_cluster::pb::cluster_server::ClusterServer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("prover coordinator started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("COORDINATOR_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    conf.merge(config_rs::Environment::with_prefix("coordinator")).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let server = Coordinator::from_config(&settings).await.expect("init server error");
    let addr = format!("{}:{:?}", settings.listenaddr, settings.port).parse().unwrap();
    grpc_run(server, addr).await.unwrap()
}

async fn grpc_run(mut grpc: Coordinator, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting gprc service");

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let on_leave = grpc.on_leave();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Ctrl-c received, shutting down");
        tx.send(()).ok();
    });

    tonic::transport::Server::builder()
        .add_service(ClusterServer::new(grpc))
        .serve_with_shutdown(addr, async {
            rx.await.ok();
        })
        .await?;

    log::info!("Shutting down, waiting for final clear");
    on_leave.leave().await;
    log::info!("Shutted down");
    Ok(())
}
