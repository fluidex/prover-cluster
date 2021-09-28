use crate::coordinator::config;
use crate::pb::{
    Circuit as pbCircuit, PollTaskRequest, RegisterRequest, RegisterResponse, SubmitProofRequest, SubmitProofResponse, Task as pbTask,
};
use bellman_ce::pairing::bn256::Bn256;
use bellman_ce::plonk::better_cs::cs::PlonkCsWidth4WithNextStepParams;
use bellman_ce::plonk::better_cs::keys::{Proof, VerificationKey};
use fluidex_common::db::models::{tablenames, task};
use fluidex_common::db::{DbType, PoolOptions};
use std::collections::HashMap;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct Controller {
    db_pool: sqlx::Pool<DbType>,
    proving_order: config::ProvingOrder,
    circuits: HashMap<String, Circuit>,
    // tasks: BTreeMap<String, Task>, // use cache if we meet performance bottle neck
}

#[derive(Debug)]
pub struct Circuit {
    vk: VerificationKey<Bn256, PlonkCsWidth4WithNextStepParams>,
}

impl Controller {
    pub async fn from_config(config: &config::Settings) -> anyhow::Result<Self> {
        let db_pool = PoolOptions::new().connect(&config.db).await?;
        let mut circuits = HashMap::new();
        for (name, circuit) in &config.circuits {
            circuits.insert(
                name.to_lowercase(),
                Circuit {
                    vk: plonkit::reader::load_verification_key::<Bn256>(&circuit.vk),
                },
            );
        }

        Ok(Self {
            db_pool,
            proving_order: config.proving_order,
            circuits,
            // tasks: BTreeMap::new(),
        })
    }

    pub async fn register(&mut self, request: RegisterRequest) -> Result<RegisterResponse, Status> {
        Ok(RegisterResponse {
            prover_id: format!("{}-{}", request.hostname, chrono::Utc::now().timestamp_millis()),
        })
    }

    pub async fn poll_task(&mut self, request: PollTaskRequest) -> Result<pbTask, Status> {
        let circuit = pbCircuit::from_i32(request.circuit).ok_or_else(|| Status::new(Code::InvalidArgument, "unknown circuit"))?;

        let task = self.query_idle_task(circuit).await?;
        log::debug!("{:?}", task);
        match task {
            None => Err(Status::new(Code::ResourceExhausted, "no task ready to prove")),
            Some(t) => {
                log::debug!("task input: {:?}", t.input.to_string());

                // self.tasks.remove(&t.task_id);
                self.assign_task(t.clone().task_id, request.prover_id).await.unwrap();
                Ok(pbTask {
                    circuit: request.circuit,
                    id: t.clone().task_id,
                    input: serde_json::to_vec(&t.input).unwrap(),
                    output: t.output.map(|o| serde_json::to_vec(&o).unwrap()),
                })
            }
        }
    }

    async fn query_idle_task(&mut self, circuit: pbCircuit) -> Result<Option<task::Task>, Status> {
        let order = match self.proving_order {
            config::ProvingOrder::Oldest => "ASC",
            config::ProvingOrder::Latest => "DESC",
        };
        let query = format!(
            "select task_id, circuit, block_id, input, output, public_input, proof, status, prover_id, created_time, updated_time
            from {}
            where circuit = $1 and status = $2
            order by block_id {}",
            tablenames::TASK,
            order
        );
        sqlx::query_as::<_, task::Task>(&query)
            .bind(task::CircuitType::from(circuit))
            .bind(task::TaskStatus::Inited)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| {
                log::error!("db query idle task: {:?}", e);
                Status::new(Code::Internal, "db query idle task")
            })
    }

    pub async fn submit_proof(&mut self, req: SubmitProofRequest) -> Result<SubmitProofResponse, Status> {
        let pb_circuit = pbCircuit::from_i32(req.circuit).unwrap();
        let proof = Proof::<Bn256, PlonkCsWidth4WithNextStepParams>::read(req.proof.as_slice()).unwrap();
        let circuit = self
            .circuits
            .get(pb_circuit.to_str())
            .unwrap_or_else(|| panic!("Uninitialized Circuit {:?} in Config file", pb_circuit));

        if !plonkit::plonk::verify(&circuit.vk, &proof).unwrap() {
            return Ok(SubmitProofResponse { valid: false });
        }

        self.store_proof(req, proof).await.unwrap();
        Ok(SubmitProofResponse { valid: true })
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn assign_task(&mut self, task_id: String, mut prover_id: String) -> anyhow::Result<()> {
        prover_id.truncate(30);
        let stmt = format!("update {} set prover_id = $1, status = $2 where task_id = $3", tablenames::TASK);
        sqlx::query(&stmt)
            .bind(prover_id)
            .bind(task::TaskStatus::Proving)
            .bind(task_id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    // Failure is acceptable here. We can re-assign the task to another prover later.
    async fn store_proof(&mut self, req: SubmitProofRequest, proof: Proof<Bn256, PlonkCsWidth4WithNextStepParams>) -> anyhow::Result<()> {
        let (public_input, proof) = bellman_vk_codegen::serialize_proof(&proof);
        let public_input = serde_json::ser::to_vec(&public_input)?;
        let proof = serde_json::ser::to_vec(&proof)?;

        let stmt = format!(
            "update {} set public_input = $1, proof = $2, prover_id = $3, status = $4 where task_id = $5",
            tablenames::TASK
        );
        let mut prover_id = req.prover_id.clone();
        prover_id.truncate(30);
        sqlx::query(&stmt)
            .bind(public_input)
            .bind(proof)
            .bind(prover_id)
            .bind(task::TaskStatus::Proved)
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
