use fluidex_common::non_blocking_tracing;
use futures::{channel::mpsc, SinkExt};
use prover_cluster::client::config;
use prover_cluster::client::watch::{WatchRequest, Watcher};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let _guard = non_blocking_tracing::setup();
    log::info!("prover client started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CLIENT_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    conf.merge(config_rs::Environment::with_prefix("client")).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let poll_interval = settings.poll_interval();
    let mut watcher = Watcher::from_config(&settings).await.expect("init watcher error");
    let (req_sender, req_receiver) = mpsc::channel(256);
    tokio::spawn(async move { watcher.run(req_receiver).await });

    req_sender
        .clone()
        .send(WatchRequest::Register)
        .await
        .expect("watch receiver dropped");

    let mut timer = tokio::time::interval(poll_interval);
    loop {
        timer.tick().await;
        req_sender
            .clone()
            .send(WatchRequest::PollTask)
            .await
            .expect("watch receiver dropped");
    }
}
