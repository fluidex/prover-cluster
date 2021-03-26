use crate::coordinator::Settings;
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Coordinator {
    pub addr: SocketAddr,
}

impl Coordinator {
    pub fn from_config(_config: &Settings) -> Self {
        Self {
            addr: "[::1]:50051".parse().unwrap(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

#[tonic::async_trait]
impl Cluster for Coordinator {
    async fn poll_task(&self, _request: Request<PollTaskRequest>) -> Result<Response<Task>, Status> {
        unimplemented!()
    }

    async fn submit_proof(&self, _request: Request<SubmitProofRequest>) -> Result<Response<SubmitProofResponse>, Status> {
        unimplemented!()
    }
}
