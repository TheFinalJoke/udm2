use crate::db::executor::GenQueries;
use crate::db::DbConnection;
use crate::db::DbMetaData;
use crate::rpc_types::server::udm_service_server::UdmService;
use crate::rpc_types::server::udm_service_server::UdmServiceServer;
use crate::rpc_types::service_types::AddFluidRegulatorRequest;
use crate::rpc_types::service_types::AddFluidRegulatorResponse;
use crate::rpc_types::service_types::ResetRequest;
use crate::rpc_types::service_types::ResetResponse;
use crate::rpc_types::service_types::ServiceResponse;
use crate::UdmResult;
use anyhow::Result;
use log;
use sea_query::PostgresQueryBuilder;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;
tonic::include_proto!("server");

pub struct DaemonServerContext {
    pub connection: Box<dyn DbConnection>,
    pub addr: SocketAddr,
    pub metadata: DbMetaData,
}

impl DaemonServerContext {
    pub fn new(connection: Box<dyn DbConnection>, addr: SocketAddr, metadata: DbMetaData) -> Self {
        Self {
            connection,
            addr,
            metadata,
        }
    }
}
#[tonic::async_trait]
impl UdmService for DaemonServerContext {
    async fn add_fluid_regulator(
        &self,
        request: Request<AddFluidRegulatorRequest>,
    ) -> Result<Response<AddFluidRegulatorResponse>, Status> {
        log::debug!("Got request {:?}", request);
        let fr = request
            .into_inner()
            .fluid
            .ok_or_else(|| Status::cancelled("Invalid request to add fluid regulator"))?;
        let query = fr.gen_insert_query().to_string(PostgresQueryBuilder);
        let input_result = self.connection.insert(query).await;
        match input_result {
            Ok(fr_id) => {
                let fr_response = AddFluidRegulatorResponse { fr_id }.to_response();
                Ok(fr_response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to insert into database: {}",
                e
            ))),
        }
    }

    async fn reset_db(
        &self,
        request: Request<ResetRequest>,
    ) -> Result<Response<ResetResponse>, Status> {
        log::info!("Got request {:?}", request);
        let dropped_result = self.connection.truncate_schema().await; 
        log::info!("the dropped Result {:?}", &dropped_result);
        match dropped_result {
            Ok(_) => {
                log::info!("Successfully dropped rows");
                Ok(
                    ResetResponse{}.to_response()
                )
            },
            Err(err) => {
                Err(Status::cancelled(format!("Failed to drop rows: {}", err.to_string())))
            }
        }
    }
}

pub async fn start_server(
    service: UdmServiceServer<DaemonServerContext>,
    addr: SocketAddr,
) -> UdmResult<()> {
    log::info!("Running Udm Service on {:?}", &addr);
    let _ = Server::builder().add_service(service).serve(addr).await;
    Ok(())
}
