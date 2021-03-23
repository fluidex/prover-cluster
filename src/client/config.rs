use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub prover_id: String,
    pub upstream: String,
    pub poll_interval: u64,
}
