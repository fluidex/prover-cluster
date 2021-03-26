use crate::coordinator::Settings;
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct Coordinator {
    pub addr: SocketAddr,
}

impl Coordinator {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            addr: format!("[::1]:{:?}", config.port).parse().unwrap(),
        }
    }
}

#[tonic::async_trait]
impl Cluster for Coordinator {
    async fn poll_task(&self, _request: Request<PollTaskRequest>) -> Result<Response<Task>, Status> {
        Ok(Response::new(Task {
            id: 1.to_string(),
            circuit: 1,
            witness: vec![],
        }))
        // unimplemented!()
    }

    async fn submit_proof(&self, _request: Request<SubmitProofRequest>) -> Result<Response<SubmitProofResponse>, Status> {
        Ok(Response::new(SubmitProofResponse { valid: true }))
        // unimplemented!()
    }
}
