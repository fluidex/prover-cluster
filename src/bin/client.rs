use prover_cluster::config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("prover client started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CLIENT_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: config::client::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    unimplemented!();
}
