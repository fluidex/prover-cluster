use crate::pb::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub prover_id: String,
    pub circuit: String,
    pub upstream: String,
    pub poll_interval: u64,
}

impl Settings {
    /// Converts `self.poll_interval` into `Duration`.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval)
    }

    pub fn circuit(&self) -> Circuit {
        match self.circuit.as_ref() {
            "Block" | "block" => Circuit::Block,
            _ => panic!("unknown circuit: {:?}", &self.circuit),
        }
    }
}
