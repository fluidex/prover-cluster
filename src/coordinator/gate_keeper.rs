use crate::coordinator::Settings;
use std::collections::BTreeMap;
use crate::pb::*;


#[derive(Debug, Clone)]
pub struct GateKeeper {
    tasks: BTreeMap<String, Task>,
}

impl GateKeeper {
    pub fn from_config(_config: &Settings) -> Self {
        Self {
        	tasks: BTreeMap::new(),
        }
    }

    // TODO: circuit type
    pub fn fetch_task(&self, circuit: i32) -> bool {
        let tasks: BTreeMap<&String, &Task> = self.tasks.iter().filter(|(_id, t)| t.circuit == circuit).collect();
        tasks.iter().next()
    }
}
