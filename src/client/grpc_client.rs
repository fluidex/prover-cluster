use crate::client::{Proof, Settings, Task};

pub struct GrpcClient {}

impl GrpcClient {
    pub fn from_config(_config: &Settings) -> Self {
        Self {}
    }

    pub async fn poll_task(&self) -> Result<Task, anyhow::Error> {
        Ok(Task {})
    }

    pub async fn submit(&self, task: Task, proof: Proof) -> Result<(), anyhow::Error> {
        log::info!("submit result for task: {:?}", task);
        log::debug!("proof: {:?}", proof);
        // if error, log error here instead of outer. because of we want an async submission.
        Ok(())
    }
}
