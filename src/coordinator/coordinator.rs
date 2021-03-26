use crate::coordinator::{GateKeeper, Settings};
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

// TODO: witness generator
// TODO: fetcher/dispatcher
// TODO: auto clean too old entries

#[derive(Debug, Clone)]
pub struct Coordinator {
    pub addr: SocketAddr,
    gate_keeper: GateKeeper,
}

impl Coordinator {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            addr: format!("[::1]:{:?}", config.port).parse().unwrap(),
            gate_keeper: GateKeeper::from_config(config),
        }
    }
}

// TODO: atomic
#[tonic::async_trait]
impl Cluster for Coordinator {
    async fn poll_task(&self, request: Request<PollTaskRequest>) -> Result<Response<Task>, Status> {
        let request = request.into_inner();
        let circuit =
            Circuit::from_i32(request.circuit).ok_or_else(|| tonic::Status::new(tonic::Code::InvalidArgument, "unknown circuit"))?;
        match self.gate_keeper.fetch_task(circuit) {
            None => Err(tonic::Status::new(tonic::Code::Unknown, "no task ready to prove")),
            Some((task_id, task)) => {
                self.gate_keeper.assign(request.prover_id, task_id, task.clone());
                Ok(Response::new(task))
            }
        }
    }

    async fn submit_proof(&self, _request: Request<SubmitProofRequest>) -> Result<Response<SubmitProofResponse>, Status> {
        // TODO: validate proof

        // TODO: mark task

        Ok(Response::new(SubmitProofResponse { valid: true }))
    }
}
