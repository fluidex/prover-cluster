use crate::pb::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub prover_id: String,
    pub upstream: String,
    pub poll_interval: u64,
    pub circuit: String,
    pub r1cs: String,
    pub srs_monomial_form: String,
    pub srs_lagrange_form: String,
    pub db: String,
    pub witgen: WitGen,
}

impl Settings {
    /// Converts `self.poll_interval` into `Duration`.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval)
    }

    pub fn circuit(&self) -> Circuit {
        let circuit = self.circuit.as_str();
        match circuit {
            "Block" | "block" => Circuit::Block,
            _ => panic!("unknown circuit: {:?}", circuit),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WitGen {
    pub interval: u64,
    pub circuits: HashMap<String, String>,
}

impl WitGen {
    /// Converts `self.interval` into `Duration`.
    pub fn interval(&self) -> Duration {
        Duration::from_millis(self.interval)
    }
}
