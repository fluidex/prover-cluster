use prover_cluster::client::config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("prover client started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CLIENT_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    unimplemented!();

    // let client = EthereumGateway::from_config(&settings);

    // let (eth_req_sender, eth_req_receiver) = mpsc::channel(256);

    // let eth_client = EthHttpClient::new(client, settings.contracts.contract_addr);
    // let watcher = EthWatch::new(eth_client, settings.eth_watch.confirmations_for_eth_event);

    // main_runtime.spawn(watcher.run(eth_req_receiver));
    // main_runtime.block_on(async move {
    //     let mut timer = time::interval(Duration::from_secs(1));

    //     loop {
    //         timer.tick().await;
    //         eth_req_sender
    //             .clone()
    //             .send(EthWatchRequest::PollETHNode)
    //             .await
    //             .expect("ETH watch receiver dropped");
    //     }
    // });
}
