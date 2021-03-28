use crate::coordinator::Settings;
use crate::pb::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Controller {
    tasks: BTreeMap<String, Task>,
}

impl Controller {
    // TODO: return error
    pub fn from_config(_config: &Settings) -> Self {
        Self { tasks: BTreeMap::new() }
    }

    pub fn fetch_task(&mut self, circuit: Circuit) -> Option<(String, Task)> {
        let tasks: BTreeMap<String, Task> = self
            .tasks
            .clone()
            .into_iter()
            .filter(|(_id, t)| t.circuit == circuit as i32)
            .collect();
        tasks.into_iter().next()
    }

    pub fn assign(&mut self, _prover_id: String, _task_id: String) {}

    pub fn store_proof(&mut self, _req: SubmitProofRequest) {}
}
