use crate::coordinator::{DbType, ConnectionType, Settings};
use crate::pb::*;
use std::collections::BTreeMap;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct Controller {
    tasks: BTreeMap<String, Task>,
    db_pool: sqlx::Pool<DbType>,
    // TODO:
    dbConn: ConnectionType,
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
        match tasks.into_iter().next() {
            None => Err(Status::new(Code::ResourceExhausted, "no task ready to prove")),
            Some((task_id, task)) => {
                self.tasks.remove(&task_id);
                self.assign_task(task_id, request.prover_id);
                Ok(task)
            }
        }
    }

    pub fn submit_proof(&mut self, req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        // TODO: validate proof

        self.store_proof(req);

        Ok(SubmitProofResponse { valid: true })
    }

    // Failure is acceptable. We can re-assign the task to another prover later.
    fn assign_task(&mut self, _task_id: String, _prover_id: String) {
        unimplemented!()
    }

    // Failure is acceptable. We can re-assign the task to another prover later.
    fn store_proof(&mut self, _req: SubmitProofRequest) {
        unimplemented!()
    }
}
