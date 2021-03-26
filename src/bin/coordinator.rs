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

    let coordinator = Coordinator::from_config(&settings);
    Server::builder().add_service(ClusterServer::new(coordinator)).serve(coordinator.addr())/*.await*/;
}
