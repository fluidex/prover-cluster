use crate::client::{Proof, Settings};
use crate::pb::cluster_client::ClusterClient;
use crate::pb::*;
use anyhow::anyhow;

pub struct GrpcClient {
    id: String,
    upstream: String,
}

impl GrpcClient {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            id: config.prover_id.clone(),
            upstream: config.upstream.clone(),
        }
    }

    // TODO:
    pub async fn poll_task(&self) -> Result<Task, anyhow::Error> {
        Ok(Task {
            id: "task_id".to_string(),
            circuit: "circuit".to_string(),
            witness: serde_json::ser::to_vec("witness").unwrap(),
        })
    }

    pub async fn submit(&self, task_id: &str, proof: Proof) -> Result<(), anyhow::Error> {
        let mut client = ClusterClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(SubmitProofRequest {
            prover_id: self.id.clone(),
            task_id: task_id.to_string(),
            proof: serde_json::ser::to_vec(&proof).unwrap(),
            signature: "".into(), // TODO: remove and use TLS certificates
            timestamp: chrono::Utc::now().timestamp_millis(),
        });

        log::info!("prover({:?}) submiting result for task({:?})", self.id, task_id);
        log::debug!("proof: {:?}", proof);

        // If error, log error here instead of outer. Because we want an async submission.
        match client.submit_proof(request).await {
            Ok(_) => {
                log::info!("prover({:?}) submit result for task({:?}) successfully", self.id, task_id);
                Ok(())
            }
            Err(e) => {
                log::error!("prover({:?}) submit result for task({:?}) error {:?}", self.id, task_id, e);
                Err(anyhow!(e))
            }
        }
    }
}
