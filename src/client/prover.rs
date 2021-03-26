use crate::client::Settings;
use crate::{pb, pb::*};
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};
use plonkit::plonk::SetupForProver;
use std::{thread, time};

pub struct Prover {
    circuit_name: pb::Circuit,
    setup: SetupForProver,
}

impl Prover {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            circuit_name: config.circuit(),
        }
    }

    pub async fn prove(&self, _task: &Task) -> Result<Proof<Bn256, PlonkCsWidth4WithNextStepParams>, anyhow::Error> {
        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);

        unimplemented!()
    }
}
