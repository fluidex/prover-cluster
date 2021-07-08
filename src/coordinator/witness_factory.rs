use crate::coordinator::db::{models, DbType, PoolOptions};
use crate::coordinator::Settings;
use anyhow::{anyhow, bail};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct WitnessFactory {
    db_pool: sqlx::Pool<DbType>,
    witgen_interval: Duration,
    circuits: HashMap<String, String>,
    n_workers: u64,
}

impl WitnessFactory {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let db_pool = PoolOptions::new().connect(&config.db).await?;
        let circuits = config.witgen.circuits.clone();
        log::debug!("{:?}", circuits);

        // check file existence
        for (k, v) in &circuits {
            log::debug!("circuit:{}, path {}", k, v);
            assert!(Path::new(v).exists(), "circuit path doesn't exist: {}", v);
        }

        Ok(Self {
            db_pool,
            witgen_interval: config.witgen.interval(),
            circuits,
            n_workers: config.witgen.n_workers,
        })
    }

    pub async fn run(self) {
        let mut timer = tokio::time::interval(self.witgen_interval);

        // TODO: refactor to worker_pool
        loop {
            timer.tick().await;
            log::debug!("ticktock!");
            if let Err(e) = self.clone().run_inner().await {
                log::error!("{}", e);
            };
        }
    }

    async fn run_inner(mut self) -> Result<(), anyhow::Error> {
        let tasks = self.claim_tasks().await.map_err(|e| anyhow!("claim_tasks error: {:?}", e))?;
        for task in tasks {
            let core = self.clone();
            tokio::spawn(async move {
                if let Err(e) = core.witgen(task).await {
                    log::error!("{}", e);
                };
            });
        }
        Ok(())
    }

    async fn witgen(mut self, task: models::Task) -> Result<(), anyhow::Error> {
        log::info!("generating witness for task {:?}", task.task_id);

        // create temp dir
        let dir = tempdir().map_err(|_| anyhow!("create tempdir in std::env::temp_dir()"))?;
        log::info!("process in tempdir path: {:?}", dir.path());

        let inputjson = format!("{}", task.input);
        log::debug!("inputjson content: {:?}", inputjson);

        // save inputjson to file
        let inputjson_filepath = dir.path().join("input.json");
        log::debug!("inputjson_filepath: {:?}", inputjson_filepath);
        let mut inputjson_file = File::create(inputjson_filepath.clone()).map_err(|_| anyhow!("create input.json"))?;
        inputjson_file
            .write_all(inputjson.as_bytes())
            .map_err(|_| anyhow!("save input.json"))?;

        // TODO: refactor these clone/ref
        if let Some(output) = task.clone().output {
            let outputjson = format!("{}", output);
            log::debug!("outputjson content: {:?}", outputjson);

            // save outputjson to file
            let outputjson_filepath = dir.path().join("output.json");
            log::debug!("outputjson_filepath: {:?}", outputjson_filepath);
            let mut outputjson_file = File::create(outputjson_filepath).map_err(|_| anyhow!("create output.json"))?;
            outputjson_file
                .write_all(outputjson.as_bytes())
                .map_err(|_| anyhow!("save output.json"))?;
        };

        let witness_filepath = dir.path().join("witness.wtns");
        log::debug!("witness_filepath: {:?}", witness_filepath);

        // decide circuit
        let circuit_name = format!("{:?}", task.circuit).to_lowercase();
        log::debug!("circuit_name: {:?}", circuit_name);
        let circuit = if let Some(circuit) = self.circuits.get(&circuit_name) {
            circuit
        } else {
            bail!("unknown circuit: {:?}", circuit_name);
        };

        // execute circuit binary & wait for the execution
        Command::new(circuit)
            .arg(inputjson_filepath.as_os_str())
            .arg(witness_filepath.as_os_str())
            .status()
            .map_err(|e| anyhow!("failed to execute circuit binary {}", e))?;

        // read from witness
        let mut witness_file = File::open(witness_filepath).map_err(|_| anyhow!("open witness.wtns"))?;
        let mut witness = Vec::new();
        // read the whole file
        witness_file.read_to_end(&mut witness).map_err(|_| anyhow!("read witness.wtns"))?;

        // save to DB
        self.save_wtns_to_db(task.clone().task_id, witness)
            .await
            .map_err(|_| anyhow!("save witness to db"))?;

        // TODO: handle offline workers (clean up Witgening tasks)

        log::info!("task (id: {:?}) witness save successfully!", task.task_id);

        Ok(())
    }

    async fn claim_tasks(&mut self) -> Result<Vec<models::Task>, anyhow::Error> {
        let mut tx = self.db_pool.begin().await?;

        let query = format!(
            "select task_id, circuit, input, output, witness, public_input, proof, status, prover_id, created_time, updated_time
            from {}
            where status = $1
            limit {}",
            models::tablenames::TASK,
            self.n_workers,
        );

        let tasks: Vec<models::Task> = sqlx::query_as(&query).bind(models::TaskStatus::Inited).fetch_all(&mut tx).await?;

        if !tasks.is_empty() {
            let ids: Vec<String> = tasks.iter().map(|t| t.task_id.clone()).collect();
            let query_set = str_vec_to_query_set(ids);
            log::debug!("query_set: {:?}", query_set);
            let stmt = format!("update {} set status = $1 where task_id in {}", models::tablenames::TASK, query_set);
            log::debug!("stmt: {:?}", stmt);
            sqlx::query(&stmt).bind(models::TaskStatus::Witgening).execute(&mut tx).await?;
        }

        tx.commit().await?;
        Ok(tasks)
    }

    async fn save_wtns_to_db(&mut self, task_id: String, witness: Vec<u8>) -> Result<(), anyhow::Error> {
        let stmt = format!(
            "update {} set witness = $1, status = $2 where task_id = $3",
            models::tablenames::TASK
        );
        sqlx::query(&stmt)
            .bind(witness)
            .bind(models::TaskStatus::Ready)
            .bind(task_id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
}

fn str_vec_to_query_set(strs: Vec<String>) -> String {
    let mut s = "(".to_owned();
    for str_i in strs {
        s = s + "'";
        s = s + &str_i;
        s = s + "'";
        s = s + ",";
    }
    s.pop();
    s = s + ")";
    s.to_string()
}
