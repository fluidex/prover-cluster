use crate::client::{Circuit, Settings};
use anyhow::anyhow;
use bellman_ce::pairing::bn256::Bn256;
use bellman_ce::plonk::better_cs::cs::PlonkCsWidth4WithNextStepParams;
use bellman_ce::plonk::better_cs::keys::Proof;

pub struct Prover {
    circuit: Circuit,
    r1cs: plonkit::circom_circuit::R1CS<Bn256>,
    setup: plonkit::plonk::SetupForProver,
}

impl Prover {
    pub fn from_config(config: &Settings) -> Self {
        let r1cs = plonkit::reader::load_r1cs(&config.circuit.r1cs);
        let circuit = plonkit::circom_circuit::CircomCircuit {
            r1cs: r1cs.clone(),
            witness: None,
            wire_mapping: None,
            aux_offset: plonkit::plonk::AUX_OFFSET,
        };
        let setup = plonkit::plonk::SetupForProver::prepare_setup_for_prover(
            circuit,
            plonkit::reader::load_key_monomial_form(&config.srs_monomial_form),
            plonkit::reader::maybe_load_key_lagrange_form(config.circuit.srs_lagrange_form.clone()),
        )
        .expect("setup prepare err");

        Self {
            circuit: config.circuit.clone(),
            r1cs,
            setup,
        }
    }

    pub async fn prove(&self, circuit: &str, witness: Vec<u8>) -> Result<Proof<Bn256, PlonkCsWidth4WithNextStepParams>, anyhow::Error> {
        if !self.circuit.is_supported(circuit) {
            log::debug!("unsupported  task circuit: {}", circuit);
            return Err(anyhow!(format!("unsupported task circuit: {}", circuit)));
        }

        let witness = plonkit::reader::load_witness_from_array::<Bn256>(witness).map_err(|e| anyhow!("load witness error: {:?}", e))?;
        let circuit = plonkit::circom_circuit::CircomCircuit {
            r1cs: self.r1cs.clone(),
            witness: Some(witness),
            wire_mapping: None,
            aux_offset: plonkit::plonk::AUX_OFFSET,
        };
        self.setup.prove(circuit, "keccak").map_err(|e| anyhow!("{:?}", e))
    }
}
