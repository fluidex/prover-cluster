use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub prover_id: String,
    pub upstream: String,
    pub poll_interval: u64,
    pub srs_monomial_form: String,
    pub circuit: Circuit,
}

impl Settings {
    /// Converts `self.poll_interval` into `Duration`.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Circuit {
    pub name: String,
    pub bin: String,
    pub r1cs: String,
    pub srs_lagrange_form: Option<String>,
}

impl Circuit {
    pub fn is_supported(&self, circuit_type: &str) -> bool {
        self.name.to_lowercase() == circuit_type.to_lowercase()
    }
}
