use crate::client::Settings;
use crate::pb::Task;
use anyhow::{anyhow, bail};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct WitnessGenerator {
    circuit: crate::client::config::Circuit,
}

impl WitnessGenerator {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            circuit: config.circuit.clone(),
        }
    }

    pub async fn witgen(&self, task: &Task) -> Result<Vec<u8>, anyhow::Error> {
        // create temp dir
        let dir = tempdir().map_err(|_| anyhow!("create tempdir in std::env::temp_dir()"))?;
        log::info!("process in tempdir path: {:?}", dir.path());

        // save inputjson to file
        let input_file_path = dir.path().join("input.json");
        log::debug!("input_file_path: {:?}", input_file_path);
        let mut inputjson_file = File::create(input_file_path.clone()).map_err(|_| anyhow!("create input.json"))?;
        inputjson_file.write_all(&task.input).map_err(|_| anyhow!("save input.json"))?;

        if let Some(output) = &task.output {
            // save outputjson to file
            let output_file_path = dir.path().join("output.json");
            log::debug!("output_file_path: {:?}", output_file_path);
            let mut outputjson_file = File::create(output_file_path).map_err(|_| anyhow!("create output.json"))?;
            outputjson_file.write_all(output).map_err(|_| anyhow!("save output.json"))?;
        };

        let witness_filepath = dir.path().join("witness.wtns");
        log::debug!("witness_filepath: {:?}", witness_filepath);

        if !self.circuit.is_supported(&task.circuit) {
            bail!("unknown circuit: {}", task.circuit);
        }

        // execute circuit binary & wait for the execution
        Command::new(&self.circuit.bin)
            .arg(input_file_path.as_os_str())
            .arg(witness_filepath.as_os_str())
            .status()
            .map_err(|e| anyhow!("failed to execute circuit binary {}", e))?;

        // read from witness
        let mut witness_file = File::open(witness_filepath).map_err(|_| anyhow!("open witness.wtns"))?;
        let mut witness = Vec::new();
        // read the whole file
        witness_file.read_to_end(&mut witness).map_err(|_| anyhow!("read witness.wtns"))?;

        Ok(witness)
    }
}
