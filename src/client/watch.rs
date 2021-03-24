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
        // try prove_and_submit first if there's any current task running
        self.prover.prove_current().await;

        while let Some(request) = watch_req.next().await {
            match request {
                WatchRequest::PollTask => {
                    log::debug!("poll task from coordinator");
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum WatchRequest {
    PollTask,
}
