use std::{thread, time};

pub struct Prover {}

impl Default for Prover {
    fn default() -> Self {
        Self::new()
    }
}

impl Prover {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn prove(&self) -> Result<u64, anyhow::Error> {
        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);
        Ok(1)
    }
}
