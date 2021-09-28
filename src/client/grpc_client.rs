use crate::client::Settings;
use crate::pb::cluster_client::ClusterClient;
use crate::pb::*;
use anyhow::anyhow;
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};

#[derive(Clone)]
pub struct GrpcClient {
    id: String,
    circuit: Circuit,
    upstream: String,
}

impl GrpcClient {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            id: config.prover_id.clone(),
            circuit: config.circuit.clone().into(),
            upstream: config.upstream.clone(),
        }
    }

    pub async fn register(&mut self) -> Result<(), anyhow::Error> {
        let hostname = gethostname();
        log::info!("register client for hostname {}", &hostname);
        let request = tonic::Request::new(RegisterRequest { hostname });

        let mut client = ClusterClient::connect(self.upstream.clone()).await?;
        match client.register(request).await {
            Ok(t) => {
                let prover_id = t.into_inner().prover_id;
                log::info!("set client prover_id {}", &prover_id);
                self.id = prover_id;
                Ok(())
            }
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn poll_task(&self) -> Result<Task, anyhow::Error> {
        let mut client = ClusterClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(PollTaskRequest {
            prover_id: self.id.clone(),
            circuit: self.circuit as i32,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });

        log::info!("prover({}) polling task", self.id);
        match client.poll_task(request).await {
            Ok(t) => Ok(t.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn submit(
        &self,
        task_id: &str,
        proof: Proof<Bn256, PlonkCsWidth4WithNextStepParams>,
    ) -> Result<SubmitProofResponse, anyhow::Error> {
        let mut proof_buffer = vec![];
        proof.write(&mut proof_buffer).unwrap();
        let mut client = ClusterClient::connect(self.upstream.clone()).await?;
        let request = tonic::Request::new(SubmitProofRequest {
            prover_id: self.id.clone(),
            task_id: task_id.to_string(),
            circuit: self.circuit as i32,
            proof: proof_buffer,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });

        log::info!("prover({:?}) submiting result for task({:?})", self.id, task_id);
        log::debug!("proof: {:?}", proof);

        match client.submit_proof(request).await {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

fn gethostname() -> String {
    let mut buf = [0u8; 64];
    let hostname = nix::unistd::gethostname(&mut buf).expect("Failed getting hostname");
    hostname.to_string_lossy().into_owned()
}
