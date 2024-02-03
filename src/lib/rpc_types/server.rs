use log;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use crate::db::DbConnection;
use crate::rpc_types::server as udm_server;
use crate::rpc_types::service_types;
use crate::UdmResult;

tonic::include_proto!("server");

pub struct DaemonServer {
    pub connection: Box<dyn DbConnection>,
    pub addr: SocketAddr,
}

impl DaemonServer {
    pub fn new(connection: Box<dyn DbConnection>, addr: SocketAddr) -> Self {
        Self { connection, addr }
    }
}
#[tonic::async_trait]
impl udm_server::udm_service_server::UdmService for DaemonServer {
    async fn add_fluid_regulator(
        &self,
        _request: Request<service_types::AddFluidRegulatorRequest>,
    ) -> Result<Response<service_types::AddFluidRegulatorResponse>, Status> {
        let reply = service_types::AddFluidRegulatorResponse { fu_id: 132 };
        Ok(Response::new(reply))
    }
}

pub async fn start_server(
    service: udm_service_server::UdmServiceServer<DaemonServer>,
    addr: SocketAddr,
) -> UdmResult<()> {
    log::info!("Running Udm Service on {:?}", &addr);
    let _ = Server::builder().add_service(service).serve(addr).await;
    Ok(())
}
