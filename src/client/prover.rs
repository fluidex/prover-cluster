use crate::client::Settings;
use crate::{pb, pb::*};
use anyhow::anyhow;
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};

pub struct Prover {
    circuit_type: pb::Circuit,
    r1cs: plonkit::circom_circuit::R1CS<Bn256>,
    setup: plonkit::plonk::SetupForProver<Bn256>,
}

impl Prover {
    pub fn from_config(config: &Settings) -> Self {
        let r1cs = plonkit::reader::load_r1cs(&config.r1cs);
        let circuit = plonkit::circom_circuit::CircomCircuit {
            r1cs: r1cs.clone(),
            witness: None,
            wire_mapping: None,
            aux_offset: plonkit::plonk::AUX_OFFSET,
        };
        let setup = plonkit::plonk::SetupForProver::prepare_setup_for_prover(
            circuit,
            plonkit::reader::load_key_monomial_form(&config.srs_monomial_form),
            plonkit::reader::maybe_load_key_lagrange_form(Some(config.srs_lagrange_form.clone())),
        )
        .expect("setup prepare err");

        Self {
            circuit_type: config.circuit(),
            r1cs,
            setup,
        }
    }

    pub async fn prove(&self, task: &Task) -> Result<Proof<Bn256, PlonkCsWidth4WithNextStepParams>, anyhow::Error> {
        log::info!("proving task id: {:?}", task.id);
        if task.circuit != (self.circuit_type as i32) {
            log::debug!("circuit_id: {:?}", task.circuit);
            log::debug!("circuit parsing result: {:?}", pb::Circuit::from_i32(task.circuit));
            return Err(anyhow!("unsupported task circuit!"));
        }

        let witness = plonkit::reader::load_witness_from_array::<Bn256>(task.witness.clone()).expect("load witness.");
        let circuit = plonkit::circom_circuit::CircomCircuit {
            r1cs: self.r1cs.clone(),
            witness: Some(witness),
            wire_mapping: None,
            aux_offset: plonkit::plonk::AUX_OFFSET,
        };
        self.setup.prove(circuit).map_err(|e| anyhow!("{:?}", e))
    }
}
