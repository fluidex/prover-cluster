use crate::coordinator::{Controller, Settings};
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use tonic::{Request, Response, Status};

// TODO: witness generator
// TODO: fetcher/dispatcher
// TODO: auto clean too old entries

#[derive(Debug)]
pub struct Coordinator {
    controller: Controller,
}

impl Coordinator {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            controller: Controller::from_config(config),
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
        match self.controller.fetch_task(circuit) {
            None => Err(tonic::Status::new(tonic::Code::ResourceExhausted, "no task ready to prove")),
            Some((task_id, task)) => {
                self.controller.assign(request.prover_id, task_id);
                Ok(Response::new(task))
            }
        }
    }

    async fn submit_proof(&self, request: Request<SubmitProofRequest>) -> Result<Response<SubmitProofResponse>, Status> {
        let request = request.into_inner();

        // TODO: validate proof

        self.controller.store_proof(request);

        Ok(Response::new(SubmitProofResponse { valid: true }))
    }
}
