use crate::client::{GrpcClient, Prover, Settings};
use futures::{channel::mpsc, StreamExt};

pub struct Watcher {
    prover: Prover,
    grpc_client: GrpcClient,
}

impl Watcher {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            prover: Prover::default(), // TODO: map for different circuits?
            grpc_client: GrpcClient::from_config(config),
        }
    }

    pub async fn run(/*mut*/ self, mut watch_req: mpsc::Receiver<WatchRequest>) {
        while let Some(request) = watch_req.next().await {
            // if busy
            if false {
                continue;
            }

            match request {
                WatchRequest::PollTask => {
                    log::debug!("poll task from coordinator");

                    let task = match self.grpc_client.poll_task().await {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("poll task error {:?}", e);
                            continue;
                        }
                    };

                    match self.prover.prove(&task).await {
                        Ok(proof) => {
                            self.grpc_client.submit(&task.id, proof);
                        }
                        Err(e) => log::error!("{:?}", e),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum WatchRequest {
    PollTask,
}
