use crate::client::Settings;
use crate::{pb, pb::*};
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};
use std::{thread, time};

pub struct Prover {
    circuit_name: pb::Circuit,
    setup: plonkit::plonk::SetupForProver<Bn256>,
}

impl Prover {
    pub fn from_config(config: &Settings) -> Self {
        let circuit = plonkit::circom_circuit::CircomCircuit {
            r1cs: plonkit::reader::load_r1cs(&config.r1cs),
            witness: None,
            wire_mapping: None,
            aux_offset: plonkit::plonk::AUX_OFFSET,
        };
        let setup = plonkit::plonk::SetupForProver::prepare_setup_for_prover(
            circuit.clone(),
            plonkit::reader::load_key_monomial_form(&config.srs_monomial_form),
            plonkit::reader::maybe_load_key_lagrange_form(Some(config.srs_lagrange_form.clone())),
        )
        .expect("prepare err");

        Self {
            circuit_name: config.circuit(),
            setup: setup,
        }
    }

    pub async fn prove(&self, _task: &Task) -> Result<Proof<Bn256, PlonkCsWidth4WithNextStepParams>, anyhow::Error> {
        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);

        unimplemented!()
    }
}
