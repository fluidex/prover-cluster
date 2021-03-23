use futures::{channel::mpsc, StreamExt};

pub struct Watcher {}

impl Watcher {
    pub fn new() -> Self {
        return Self {};
    }

    pub async fn run(/*mut*/ self, mut watch_req: mpsc::Receiver<WatchRequest>) {
        while let Some(request) = watch_req.next().await {
            match request {
                WatchRequest::PollTask => {
                    log::debug!("poll");
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum WatchRequest {
    PollTask,
}
