use crate::coordinator::{GateKeeper, Settings};
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

// TODO: witness generator
// TODO: fetcher/dispatcher

#[derive(Debug, Clone)]
pub struct Coordinator {
    pub addr: SocketAddr,
    pub circuit_tasks: HashMap<Circuit, Task>,
    gate_keeper: GateKeeper,
}

impl Coordinator {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            addr: format!("[::1]:{:?}", config.port).parse().unwrap(),
            circuit_tasks: HashMap::new(),
            gate_keeper: GateKeeper::new(),
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
