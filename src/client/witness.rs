use crate::client::Settings;
use anyhow::{anyhow, bail};
use fluidex_common::db::models::task;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct Witness {
    circuits: HashMap<String, String>,
}

impl Witness {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let circuits = config.witgen.circuits.clone();
        log::debug!("{:?}", circuits);

        // check file existence
        for (k, v) in &circuits {
            log::debug!("circuit:{}, path {}", k, v);
            assert!(Path::new(v).exists(), "circuit path doesn't exist: {}", v);
        }

        Ok(Self {
            circuits,
        })
    }

    pub async fn witgen(&self, task: task::Task) -> Result<Vec<u8>, anyhow::Error> {
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

        log::info!("task (id: {:?}) witness save successfully!", task.task_id);
        Ok(witness)
    }
}
