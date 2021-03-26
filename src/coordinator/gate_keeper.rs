use crate::coordinator::Settings;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GateKeeper {
    tasks: BTreeMap<String, Task>,
}

impl GateKeeper {
    pub fn from_config(_config: &Settings) -> Self {
        Self {}
    }
}
