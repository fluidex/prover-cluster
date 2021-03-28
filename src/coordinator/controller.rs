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
        let circuit = Circuit::from_i32(request.circuit).ok_or_else(|| Status::new(Code::InvalidArgument, "unknown circuit"))?;

        let tasks: BTreeMap<String, Task> = self
            .tasks
            .clone()
            .into_iter()
            .filter(|(_id, t)| t.circuit == circuit as i32)
            .collect();

        // match self.controller.fetch_task(circuit) {
        match tasks.into_iter().next() {
            None => Err(Status::new(Code::ResourceExhausted, "no task ready to prove")),
            Some((task_id, task)) => {
                // self.controller.assign(request.prover_id, task_id);
                Ok(task)
            }
        }
    }

    pub fn submit_proof(&mut self, _req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        unimplemented!();

        // TODO: validate proof

        // TODO: store_proof

        // Ok(SubmitProofResponse { valid: true })
    }
}
