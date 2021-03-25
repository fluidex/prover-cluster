use crate::pb::*;
use anyhow::anyhow;
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};
use serde::{Deserialize, Serialize};
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

    pub async fn prove(&self, _task: &Task) -> Result<Proof<Bn256, PlonkCsWidth4WithNextStepParams>, anyhow::Error> {
        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);

        unimplemented!()
    }
}
