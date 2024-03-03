use crate::db::executor::GenQueries;
use crate::db::DbConnection;
use crate::db::DbMetaData;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::server::udm_service_server::UdmService;
use crate::rpc_types::server::udm_service_server::UdmServiceServer;
use crate::rpc_types::service_types::AddFluidRegulatorRequest;
use crate::rpc_types::service_types::AddFluidRegulatorResponse;
use crate::rpc_types::service_types::AddIngredientRequest;
use crate::rpc_types::service_types::AddIngredientResponse;
use crate::rpc_types::service_types::AddInstructionRequest;
use crate::rpc_types::service_types::AddInstructionResponse;
use crate::rpc_types::service_types::AddRecipeRequest;
use crate::rpc_types::service_types::AddRecipeResponse;
use crate::rpc_types::service_types::GenericRemovalResponse;
use crate::rpc_types::service_types::ModifyFluidRegulatorRequest;
use crate::rpc_types::service_types::ModifyFluidRegulatorResponse;
use crate::rpc_types::service_types::ModifyIngredientRequest;
use crate::rpc_types::service_types::ModifyIngredientResponse;
use crate::rpc_types::service_types::ModifyInstructionRequest;
use crate::rpc_types::service_types::ModifyInstructionResponse;
use crate::rpc_types::service_types::ModifyRecipeRequest;
use crate::rpc_types::service_types::ModifyRecipeResponse;
use crate::rpc_types::service_types::RemoveFluidRegulatorRequest;
use crate::rpc_types::service_types::RemoveIngredientRequest;
use crate::rpc_types::service_types::RemoveInstructionRequest;
use crate::rpc_types::service_types::RemoveRecipeRequest;
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
    async fn remove_fluid_regulator(
        &self,
        request: Request<RemoveFluidRegulatorRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        log::debug!("Got Request {:?}", request);
        let fr_id = request.into_inner().fr_id;
        let query = FluidRegulator::gen_remove_query(fr_id).to_string(PostgresQueryBuilder);
        let delete_result = self.connection.delete(query).await;
        match delete_result {
            Ok(_) => {
                let remove_response = GenericRemovalResponse {}.to_response();
                Ok(remove_response)
            }
            Err(e) => Err(Status::aborted(e.to_string())),
        }
    }
    async fn update_fluid_regulator(
        &self,
        _request: Request<ModifyFluidRegulatorRequest>,
    ) -> Result<Response<ModifyFluidRegulatorResponse>, Status> {
        todo!()
    }
    async fn add_recipe(
        &self,
        _request: Request<AddRecipeRequest>,
    ) -> Result<Response<AddRecipeResponse>, Status> {
        todo!()
    }
    async fn remove_recipe(
        &self,
        _request: Request<RemoveRecipeRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        todo!()
    }
    async fn update_recipe(
        &self,
        _request: Request<ModifyRecipeRequest>,
    ) -> Result<Response<ModifyRecipeResponse>, Status> {
        todo!()
    }
    async fn add_instruction(
        &self,
        _request: Request<AddInstructionRequest>,
    ) -> Result<Response<AddInstructionResponse>, Status> {
        todo!()
    }
    async fn remove_instruction(
        &self,
        _request: Request<RemoveInstructionRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        todo!()
    }
    async fn update_instruction(
        &self,
        _request: Request<ModifyInstructionRequest>,
    ) -> Result<Response<ModifyInstructionResponse>, Status> {
        todo!()
    }
    async fn add_ingredient(
        &self,
        _request: Request<AddIngredientRequest>,
    ) -> Result<Response<AddIngredientResponse>, Status> {
        todo!()
    }
    async fn remove_ingredient(
        &self,
        _request: Request<RemoveIngredientRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        todo!()
    }
    async fn update_ingredient(
        &self,
        _request: Request<ModifyIngredientRequest>,
    ) -> Result<Response<ModifyIngredientResponse>, Status> {
        todo!()
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
                Ok(ResetResponse {}.to_response())
            }
            Err(err) => Err(Status::cancelled(format!("Failed to drop rows: {}", err))),
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
