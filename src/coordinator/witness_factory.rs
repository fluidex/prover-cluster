use crate::coordinator::db::{models, ConnectionType};
use crate::coordinator::Settings;
use sqlx::Connection;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::tempdir;

#[derive(Debug)]
pub struct WitnessFactory {
    db_conn: ConnectionType,
    witgen_interval: Duration,
    circuits: HashMap<String, String>,
}

impl WitnessFactory {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let db_conn = ConnectionType::connect(&config.db).await?;
        let circuits = config.witgen.circuits.clone();
        log::debug!("{:?}", circuits);

        // check file existence
        for (k, v) in &circuits {
            log::debug!("circuit:{}, path {}", k, v);
            assert!(Path::new(v).exists(), "circuit path doesn't exist: {}", v);
        }

        Ok(Self {
            db_conn,
            witgen_interval: config.witgen.interval(),
            circuits,
        })
    }

    pub async fn run(mut self) {
        let mut timer = tokio::time::interval(self.witgen_interval);

        // TODO: use worker_pool for multiple workers
        loop {
            timer.tick().await;
            log::debug!("ticktock!");

            let task: Option<models::Task> = if let Ok(task) = self.claim_one_task().await {
                task
            } else {
                log::error!("claim_one_task: read DB fails");
                continue;
            };
            if task.is_none() {
                continue;
            }
            let task = task.unwrap();
            log::info!("get 1 task to generate witness");

            // create temp dir
            let dir = if let Ok(dir) = tempdir() {
                dir
            } else {
                log::error!("create tempdir in std::env::temp_dir()");
                continue;
            };
            log::info!("process in tempdir path: {:?}", dir.path());

            let inputjson = format!("{}", task.input);
            log::debug!("inputjson content: {:?}", inputjson);

            // save inputjson to file
            let inputjson_filepath = dir.path().join("input.json");
            log::debug!("inputjson_filepath: {:?}", inputjson_filepath);
            let mut inputjson_file = if let Ok(inputjson_file) = File::create(inputjson_filepath.clone()) {
                inputjson_file
            } else {
                log::error!("create input.json");
                continue;
            };
            if inputjson_file.write_all(inputjson.as_bytes()).is_err() {
                log::error!("save input.json");
                continue;
            };

            let witness_filepath = dir.path().join("witness.wtns");
            log::debug!("witness_filepath: {:?}", witness_filepath);

            // decide circuit
            let circuit_name = format!("{:?}", task.circuit).to_lowercase();
            log::debug!("circuit_name: {:?}", circuit_name);
            if self.circuits.get(&circuit_name).is_none() {
                log::error!("unknown circuit: {:?}", circuit_name);
                continue;
            }
            let circuit = self.circuits.get(&circuit_name).unwrap();

            // execute circuit binary & wait for the execution
            if Command::new(circuit)
                .arg(inputjson_filepath.as_os_str())
                .arg(witness_filepath.as_os_str())
                .status()
                .is_err()
            {
                log::error!("failed to execute circuit binary");
                continue;
            };

            // read from witness
            let mut witness_file = if let Ok(witness_file) = File::open(witness_filepath) {
                witness_file
            } else {
                log::error!("open witness.wtns");
                continue;
            };
            let mut witness = Vec::new();
            // read the whole file
            if witness_file.read_to_end(&mut witness).is_err() {
                log::error!("read witness.wtns");
                continue;
            };

            // save to DB
            if self.save_wtns_to_db(task.clone().task_id, witness).await.is_err() {
                log::error!("save witness to db");
                continue;
            };

            // TODO: handle offline workers (clean up Witgening tasks)

            log::info!("task (id: {:?}) witness save successfully!", task.task_id);
        }
    }

    async fn claim_one_task(&mut self) -> Result<Option<models::Task>, anyhow::Error> {
        let mut tx = self.db_conn.begin().await?;

        let query = format!(
            "select task_id, circuit, input, witness, proof, status, prover_id, created_time, updated_time
            from {}
            where status = $1 limit 1",
            models::tablenames::TASK
        );

        let fetch_res = sqlx::query_as::<_, models::Task>(&query)
            .bind(models::TaskStatus::Inited)
            .fetch_optional(&mut tx)
            .await?;

        if let Some(ref t) = fetch_res {
            let stmt = format!("update {} set status = $1 where task_id = $2", models::tablenames::TASK);
            sqlx::query(&stmt)
                .bind(models::TaskStatus::Witgening)
                .bind(t.clone().task_id)
                .execute(&mut tx)
                .await?;
        };

        tx.commit().await?;
        Ok(fetch_res)
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
            .execute(&mut self.db_conn)
            .await?;
        Ok(())
    }
}
