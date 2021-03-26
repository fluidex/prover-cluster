use std::collections::HashMap;
use crate::pb::*;

#[derive(Debug, Clone)]
pub struct GateKeeper {
	tasks: HashMap<String, Task>,
}

impl GateKeeper {
    pub fn new() -> Self {
        Self {
        	tasks: HashMap::new(),
        }
    }
}
