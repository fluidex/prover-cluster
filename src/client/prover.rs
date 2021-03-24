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

    pub async fn prove_current(&self) -> Result<u64, anyhow::Error> {
        Ok(1)
    }
}
