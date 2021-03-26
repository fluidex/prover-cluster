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

    // TODO: circuit type
    pub fn fetch_task(&self, circuit: Circuit) -> Circuit {
        // let tasks: BTreeMap<&String, &Task> = self.tasks.iter().filter(|(_id, t)| t.circuit == circuit).collect();
        // tasks.iter().next()
        circuit
    }
}
