use crate::client::prover::Prover;
use futures::{channel::mpsc, StreamExt};

pub struct Watcher {
    prover: Prover,
}

impl Watcher {
    pub fn new() -> Self {
        Self { prover: Prover::default() }
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

                    // let task = fetch_task();

                    match self.prover.prove().await {
                        Ok(proof) => {
                            // submit
                        },
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
