use crate::coordinator::db::ConnectionType;
use crate::coordinator::Settings;
use crate::pb::*;
use sqlx::Connection;
use std::collections::BTreeMap;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct Controller {
    tasks: BTreeMap<String, Task>,
    db_conn: ConnectionType,
    // we don't need batch update
    // db_pool: sqlx::Pool<DbType>,
}

impl Controller {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let db_conn = ConnectionType::connect(&config.db).await?;
        let /*mut*/ ret = Self {
            tasks: BTreeMap::new(),
            db_conn: db_conn,
        };

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

    // Failure is acceptable here. We can re-assign the task to another prover later.
    fn assign_task(&mut self, _task_id: String, _prover_id: String) {
        unimplemented!();

        let query = format!("", tablenames::TASK);
        sqlx::query(&query)
            .bind(&asset.name)
            .bind(asset.prec_save as i16)
            .bind(asset.prec_show as i16)
            .execute(self.db_conn)
            .await?;

        Ok(())
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    fn store_proof(&mut self, _req: SubmitProofRequest) {
        unimplemented!()
    }
}
