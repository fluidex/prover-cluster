use crate::coordinator::Settings;
use crate::pb::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GateKeeper {
    tasks: HashMap<String, Task>,
}

impl GateKeeper {
    pub fn from_config(_config: &Settings) -> Self {
        Self { tasks: HashMap::new() }
    }
}
