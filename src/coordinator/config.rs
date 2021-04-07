use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub port: u64,
    pub db: String,
    pub witgen: WitGen,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WitGen {
    pub interval: u64,
    pub n_workers: u64,
    pub circuits: HashMap<String, String>,
}

impl WitGen {
    /// Converts `self.interval` into `Duration`.
    pub fn interval(&self) -> Duration {
        Duration::from_millis(self.interval)
    }
}
