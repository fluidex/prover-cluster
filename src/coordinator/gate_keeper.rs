use crate::coordinator::Settings;
use crate::pb::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GateKeeper {
    tasks: BTreeMap<String, Task>,
}

impl GateKeeper {
    pub fn from_config(_config: &Settings) -> Self {
        Self { tasks: BTreeMap::new() }
    }

    pub fn fetch_task(&self, circuit: Circuit) -> Option<(String, Task)> {
        let tasks: BTreeMap<String, Task> = self
            .tasks
            .clone()
            .into_iter()
            .filter(|(_id, t)| t.circuit == circuit as i32)
            .collect();
        tasks.into_iter().next()
    }

    pub fn assign(&self, _prover_id: String, _task_id: String) {}

    pub fn store_proof(&self, _req: SubmitProofRequest) {}
}
