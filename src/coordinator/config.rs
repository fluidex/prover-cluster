use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

fn default_addr() -> String {
    "[::1]".to_string()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum ProvingOrder {
    Oldest,
    Latest,
}

impl Default for ProvingOrder {
    fn default() -> Self {
        Self::Oldest
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    #[serde(default = "default_addr")]
    pub listenaddr: String,
    pub port: u64,
    pub db: String,
    pub circuits: HashMap<String, Circuit>,
    #[serde(default)]
    pub proving_order: ProvingOrder,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Circuit {
    pub vk: String,
}
