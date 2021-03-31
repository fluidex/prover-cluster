use crate::coordinator::db::{models, ConnectionType, MIGRATOR};
use crate::coordinator::Settings;
use crate::pb::*;
use sqlx::Connection;
// use std::collections::BTreeMap;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct Controller {
    db_conn: ConnectionType,
    // tasks: BTreeMap<String, Task>, // use cache if we meet performance bottle neck
    // db_pool: sqlx::Pool<DbType>, // we don't need batch update
}

impl Controller {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let mut db_conn = ConnectionType::connect(&config.db).await?;
        MIGRATOR.run(&mut db_conn).await?;
        Ok(Self {
            db_conn: db_conn,
            // tasks: BTreeMap::new(),
        })
    }

    pub fn poll_task(&mut self, request: PollTaskRequest) -> Result<Task, Status> {
        let circuit = Circuit::from_i32(request.circuit).ok_or_else(|| Status::new(Code::InvalidArgument, "unknown circuit"))?;

        let query = format!(
            "select task_id, circuit, witness, proof, status, prover_id, created_time, updated_time
            from {}
            where circuit = $1 and status = $2",
            models::tablenames::TASK
        );
        let task = sqlx::query_as::<_, models::Task>(&query).bind(/*self.market_load_time*/).bind(models::TaskStatus::Assigned).fetch_optional(&mut self.db_conn).await?;
        match task {
            None => Err(Status::new(Code::ResourceExhausted, "no task ready to prove")),
            Some(t) => {
                self.assign_task(t.task_id, request.prover_id);
                Ok(Task {
                    circuit: request.circuit,
                    id: t.task_id,
                    witness: hex::decode(t.witness).unwrap(),
                })
            }
        }
    }

    pub fn submit_proof(&mut self, req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        // TODO: validate proof

        self.store_proof(req);

        Ok(SubmitProofResponse { valid: true })
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn assign_task(&mut self, task_id: String, prover_id: String) -> anyhow::Result<()> {
        let stmt = format!(
            "update from {} set prover_id = $1, status = $2 where task_id = $3",
            models::tablenames::TASK
        );
        sqlx::query(&stmt)
            .bind(prover_id)
            .bind(models::TaskStatus::Assigned)
            .bind(task_id)
            .execute(&mut self.db_conn)
            .await?;
        Ok(())
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn store_proof(&mut self, req: SubmitProofRequest) -> anyhow::Result<()> {
        let stmt = format!(
            "update from {} set proof = $1, prover_id = $2, status = $3 where task_id = $4",
            models::tablenames::TASK
        );
        sqlx::query(&stmt)
            .bind(hex::encode(req.proof))
            .bind(req.prover_id)
            .bind(models::TaskStatus::Proved)
            .bind(req.task_id)
            .execute(&mut self.db_conn)
            .await?;
        Ok(())
    }
}

#[cfg(sqlxverf)]
fn sqlverf_assign_task() -> impl std::any::Any {
    let stmt = format!(
        "update from {} set prover_id = $1, status = $2 where task_id = $3",
        models::tablenames::TASK
    );
    sqlx::query!(&stmt, "prover_id", models::TaskStatus::Assigned, "task_id")
}

#[cfg(sqlxverf)]
fn sqlverf_store_proof() -> impl std::any::Any {
    let proof = vec![0xab, 0xcd];
    let stmt = format!(
        "update from {} set proof = $1, prover_id = $2, status = $3 where task_id = $4",
        models::tablenames::TASK
    );
    sqlx::query!(&stmt, hex::encode(proof), "prover_id", models::TaskStatus::Proved, "task_id")
}
