use crate::coordinator::Settings;
use crate::pb::*;
use std::collections::BTreeMap;
use tonic::Status;

#[derive(Debug, Clone)]
pub struct Controller {
    tasks: BTreeMap<String, Task>,
}

impl Controller {
    // TODO: return error
    pub fn from_config(_config: &Settings) -> Self {
        Self { tasks: BTreeMap::new() }
    }

    pub fn poll_task(&mut self, request: PollTaskRequest) -> Result<Task, Status> {
        let circuit =
            Circuit::from_i32(request.circuit).ok_or_else(|| tonic::Status::new(tonic::Code::InvalidArgument, "unknown circuit"))?;

        let tasks: BTreeMap<String, Task> = self
            .tasks
            .clone()
            .into_iter()
            .filter(|(_id, t)| t.circuit == circuit as i32)
            .collect();

        // match self.controller.fetch_task(circuit) {
        match tasks.into_iter().next() {
            None => Err(tonic::Status::new(tonic::Code::ResourceExhausted, "no task ready to prove")),
            Some((task_id, task)) => {
                // self.controller.assign(request.prover_id, task_id);
                Ok(task)
            }
        }
    }

    pub fn submit_proof(&mut self, _req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        unimplemented!();

        // TODO: validate proof

        // self.controller.store_proof(request);

        // Ok(SubmitProofResponse { valid: true })
    }
}
