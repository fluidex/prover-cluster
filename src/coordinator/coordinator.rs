use tonic::{transport::Server, Request, Response, Status};
use crate::coordinator::Settings;
use crate::pb::cluster_server::{Cluster};
use crate::pb::*;
use std::net::SocketAddr;

#[derive(Debug, Default)]
pub struct Coordinator {
    pub addr: SocketAddr,
}

impl Coordinator {
    pub fn from_config(_config: &Settings) -> Self {
        Self {
            addr: "[::1]:50051".parse().unwrap(),
        }
    }
}

#[tonic::async_trait]
impl Cluster for Coordinator {
    async fn poll_task(
        &self,
        request: Request<PollTaskRequest>,
    ) -> Result<Response<Task>, Status> {
    	unimplemented!()
    }

    async fn submit_proof(
        &self,
        request: Request<SubmitProofRequest>,
    ) -> Result<Response<SubmitProofResponse>, Status> {
    	unimplemented!()
    }
}
