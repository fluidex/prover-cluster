use prover_cluster::coordinator::{config, Coordinator};
use prover_cluster::pb::cluster_server::ClusterServer;
use tonic::transport::Server;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("prover coordinator started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("COORDINATOR_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    // let addr = format!("[::1]:{:?}", settings.port).parse().unwrap();
    // let coordinator = Coordinator::from_config(&settings);
    // Server::builder().add_service(ClusterServer::new(coordinator)).serve(addr).await?;

    // Ok(())

    let main_runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    main_runtime
        .block_on(async {
            let server = Coordinator::from_config(&settings).await
            // .expect("Init state error")
            ;
            grpc_run(server).await
        })
        .unwrap();
}

// async fn prepare() -> anyhow::Result<Coordinator> {
//     let mut grpc_stub = create_controller(settings);
//     let grpc = Coordinator::new(grpc_stub);
//     Ok(grpc)
// }

async fn grpc_run(mut grpc: Coordinator) -> Result<(), Box<dyn std::error::Error>> {
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
        .serve_with_shutdown(grpc.addr, async {
            rx.await.ok();
        })
        .await?;

    log::info!("Shutted down, wait for final clear");
    on_leave.leave().await;
    log::info!("Shutted down");
    Ok(())
}
