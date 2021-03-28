use crate::coordinator::Settings;
use crate::pb::*;
use std::collections::BTreeMap;
use tonic::{Code, Status};

#[derive(Debug, Clone)]
pub struct Controller {
    tasks: BTreeMap<String, Task>,
}

impl Controller {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let /*mut*/ ret = Self { tasks: BTreeMap::new() };

        Ok(ret)
    }

    pub fn poll_task(&mut self, request: PollTaskRequest) -> Result<Task, Status> {
        unimplemented!();

        // TODO: pop task

        // TODO: assign task
    }

    pub fn submit_proof(&mut self, _req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        unimplemented!();

        // TODO: validate proof

        // TODO: store proof

        // Ok(SubmitProofResponse { valid: true })
    }
}
