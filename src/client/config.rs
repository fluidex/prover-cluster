use crate::pb::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub prover_id: String,
    pub upstream: String,
    pub poll_interval: u64,
    pub srs_monomial_form: String,
    pub db: String,
    pub circuit: Circuit,
}

impl Settings {
    /// Converts `self.poll_interval` into `Duration`.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval)
    }

    // pub fn circuit(&self) -> Circuit {
    //     let circuit = self.circuit.as_str();
    //     match circuit {
    //         "Block" | "block" => Circuit::Block,
    //         _ => panic!("unknown circuit: {:?}", circuit),
    //     }
    // }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Circuit {
    pub name: String,
    pub bin: String,
    pub r1cs: String,
    pub vk: String,
    pub srs_lagrange_form: String,
}
