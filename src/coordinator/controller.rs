use crate::coordinator::config::*;
use crate::coordinator::db::{models, DbType, PoolOptions};
use crate::pb::*;
// use std::collections::BTreeMap;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct Controller {
    db_pool: sqlx::Pool<DbType>,
    proving_order: ProvingOrder,
    // tasks: BTreeMap<String, Task>, // use cache if we meet performance bottle neck
}

impl Controller {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let db_pool = PoolOptions::new().connect(&config.db).await?;
        Ok(Self {
            db_pool,
            proving_order: config.proving_order,
            // tasks: BTreeMap::new(),
        })
    }

    pub async fn poll_task(&mut self, request: PollTaskRequest) -> Result<Task, Status> {
        let circuit = Circuit::from_i32(request.circuit).ok_or_else(|| Status::new(Code::InvalidArgument, "unknown circuit"))?;

        let task = self.query_idle_task(circuit).await?;
        log::debug!("{:?}", task);
        match task {
            None => Err(Status::new(Code::ResourceExhausted, "no task ready to prove")),
            Some(t) => {
                log::debug!("task input: {:?}", t.input.to_string());

                // self.tasks.remove(&t.task_id);
                self.assign_task(t.clone().task_id, request.prover_id).await.unwrap();
                Ok(Task {
                    circuit: request.circuit,
                    id: t.clone().task_id,
                    witness: t.witness.unwrap(),
                })
            }
        }
    }

    async fn query_idle_task(&mut self, circuit: Circuit) -> Result<Option<models::Task>, Status> {
        let order = match self.proving_order {
            ProvingOrder::Oldest => "ASC",
            ProvingOrder::Latest => "DESC",
        };
        let query = format!(
            "select task_id, circuit, input, output, witness, public_input, proof, status, prover_id, created_time, updated_time
            from {}
            where circuit = $1 and status = $2
            order by created_time $3",
            models::tablenames::TASK
        );
        sqlx::query_as::<_, models::Task>(&query)
            .bind(models::CircuitType::from(circuit))
            .bind(models::TaskStatus::Ready)
            .bind(order)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| {
                log::error!("db query idle task: {:?}", e);
                Status::new(Code::Internal, "db query idle task")
            })
    }

    pub async fn submit_proof(&mut self, req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        // TODO: validate proof

        self.store_proof(req).await.unwrap();

        Ok(SubmitProofResponse { valid: true })
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn assign_task(&mut self, task_id: String, prover_id: String) -> anyhow::Result<()> {
        let stmt = format!(
            "update {} set prover_id = $1, status = $2 where task_id = $3",
            models::tablenames::TASK
        );
        sqlx::query(&stmt)
            .bind(prover_id)
            .bind(models::TaskStatus::Assigned)
            .bind(task_id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn store_proof(&mut self, req: SubmitProofRequest) -> anyhow::Result<()> {
        let stmt = format!(
            "update {} set public_input = $1, proof = $2, prover_id = $3, status = $4 where task_id = $5",
            models::tablenames::TASK
        );
        sqlx::query(&stmt)
            .bind(req.public_input)
            .bind(req.proof)
            .bind(req.prover_id)
            .bind(models::TaskStatus::Proved)
            .bind(req.task_id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}

#[cfg(sqlxverf)]
fn sqlverf_assign_task() -> impl std::any::Any {
    let stmt = format!(
        "update {} set prover_id = $1, status = $2 where task_id = $3",
        models::tablenames::TASK
    );
    sqlx::query!(&stmt, "prover_id", models::TaskStatus::Assigned, "task_id")
}

#[cfg(sqlxverf)]
fn sqlverf_store_proof() -> impl std::any::Any {
    let public_input = vec![0xab, 0xcd];
    let proof = vec![0xab, 0xcd];
    let stmt = format!(
        "update {} set public_input = $1, proof = $2, prover_id = $3, status = $4 where task_id = $5",
        models::tablenames::TASK
    );
    sqlx::query!(&stmt, public_input, proof, "prover_id", models::TaskStatus::Proved, "task_id")
}
