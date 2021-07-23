use crate::client::{GrpcClient, Prover, Settings};
use futures::{channel::mpsc, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Watcher {
    prover: Prover,
    grpc_client: GrpcClient,
    is_busy: AtomicBool,
}

impl Watcher {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            prover: Prover::from_config(config),
            grpc_client: GrpcClient::from_config(config),
            is_busy: AtomicBool::new(false),
        }
    }

    pub async fn run(mut self, mut watch_req: mpsc::Receiver<WatchRequest>) {
        while let Some(request) = watch_req.next().await {
            if self.is_busy.load(Ordering::SeqCst) {
                continue;
            }

            match request {
                WatchRequest::Register => {
                    log::debug!("WatchRequest::Register");
                    self.is_busy.store(true, Ordering::SeqCst);
                    if let Err(e) = self.grpc_client.register().await {
                        log::error!("register client error {:?}", e);
                    }
                    self.is_busy.store(false, Ordering::SeqCst);
                }
                WatchRequest::PollTask => {
                    log::debug!("WatchRequest::PollTask");
                    self.is_busy.store(true, Ordering::SeqCst);
                    let task = match self.grpc_client.poll_task().await {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("poll task error {:?}", e);
                            self.is_busy.store(false, Ordering::SeqCst);
                            continue;
                        }
                    };

                    match self.prover.prove(&task).await {
                        Ok(proof) => {
                            let grpc_client = self.grpc_client.clone();
                            tokio::spawn(/*move ||*/ async move {
                                match grpc_client.submit(&task.id, proof).await {
                                    Ok(resp) => {
                                        log::info!("submission for task({:?}) successful", &task.id);
                                        log::info!("task({:?}) submission result valid: {:?}", &task.id, resp.valid);
                                    }
                                    Err(e) => {
                                        log::error!("submit result for task({:?}) error {:?}", &task.id, e);
                                    }
                                };
                            });
                        }
                        Err(e) => log::error!("{:?}", e),
                    }

                    self.is_busy.store(false, Ordering::SeqCst);
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum WatchRequest {
    Register,
    PollTask,
}
