use crate::client::Settings;
use crate::pb::cluster_client::ClusterClient;
use crate::pb::*;
use anyhow::anyhow;
use bellman_ce::{
    pairing::bn256::Bn256,
    plonk::better_cs::{cs::PlonkCsWidth4WithNextStepParams, keys::Proof},
};

pub struct GrpcClient {
    id: String,
    circuit: Circuit,
    upstream: String,
}

impl GrpcClient {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            id: config.prover_id.clone(),
            circuit: config.circuit(),
            upstream: config.upstream.clone(),
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
        let (_, serialized_proof) = bellman_vk_codegen::serialize_proof(&proof);
        let mut client = ClusterClient::connect(self.upstream.clone()).await?;
        let request = tonic::Request::new(SubmitProofRequest {
            prover_id: self.id.clone(),
            task_id: task_id.to_string(),
            proof: serde_json::ser::to_vec(&serialized_proof).unwrap(),
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
