use crate::db::executor::GenQueries;
use crate::db::DbConnection;
use crate::db::DbMetaData;
use crate::db::FluidRegulationSchema;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::recipe_types::Ingredient;
use crate::rpc_types::recipe_types::Instruction;
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
use crate::rpc_types::service_types::CollectExpressions;
use crate::rpc_types::service_types::CollectFluidRegulatorsRequest;
use crate::rpc_types::service_types::CollectFluidRegulatorsResponse;
use crate::rpc_types::service_types::CollectIngredientRequest;
use crate::rpc_types::service_types::CollectIngredientResponse;
use crate::rpc_types::service_types::CollectInstructionRequest;
use crate::rpc_types::service_types::CollectInstructionResponse;
use crate::rpc_types::service_types::FetchData;
use crate::rpc_types::service_types::GenericRemovalResponse;
use crate::rpc_types::service_types::ModifyFluidRegulatorRequest;
use crate::rpc_types::service_types::ModifyFluidRegulatorResponse;
use crate::rpc_types::service_types::ModifyIngredientRequest;
use crate::rpc_types::service_types::ModifyIngredientResponse;
use crate::rpc_types::service_types::ModifyInstructionRequest;
use crate::rpc_types::service_types::ModifyInstructionResponse;
use crate::rpc_types::service_types::ModifyRecipeRequest;
use crate::rpc_types::service_types::ModifyRecipeResponse;
use crate::rpc_types::service_types::Operation;
use crate::rpc_types::service_types::RemoveFluidRegulatorRequest;
use crate::rpc_types::service_types::RemoveIngredientRequest;
use crate::rpc_types::service_types::RemoveInstructionRequest;
use crate::rpc_types::service_types::RemoveRecipeRequest;
use crate::rpc_types::service_types::ResetRequest;
use crate::rpc_types::service_types::ResetResponse;
use crate::rpc_types::service_types::ServiceResponse;
use crate::UdmResult;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use sea_query::PostgresQueryBuilder;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::IntoRequest;
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
        tracing::debug!("Got request {:?}", request);
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
        tracing::debug!("Got Request {:?}", request);
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
        request: Request<ModifyFluidRegulatorRequest>,
    ) -> Result<Response<ModifyFluidRegulatorResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let fr = request
            .into_inner()
            .fluid
            .ok_or_else(|| Status::cancelled("Invalid request to remove fluid regulator"))?;
        let query = fr.gen_update_query().to_string(PostgresQueryBuilder);
        let result = self.connection.update(query).await;
        match result {
            Ok(fr_id) => {
                let fr_response = ModifyFluidRegulatorResponse { fr_id }.to_response();
                Ok(fr_response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to update into database: {}",
                e
            ))),
        }
    }
    async fn collect_fluid_regulators(
        &self,
        request: Request<CollectFluidRegulatorsRequest>,
    ) -> Result<Response<CollectFluidRegulatorsResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let exprs = request
            .into_inner()
            .get_expressions()
            .map_err(|e| Status::cancelled(e.to_string()))?;
        let query = FluidRegulator::gen_select_query_on_fields(FluidRegulationSchema::Table, exprs)
            .to_string(PostgresQueryBuilder);
        let results = self.connection.select(query).await;
        match results {
            Ok(results) => {
                let frs: Vec<FluidRegulator> = results
                    .into_iter()
                    .map(|row| FluidRegulator::try_from(row).unwrap())
                    .collect_vec();
                tracing::info!("Successfully collected fluid regulators");
                tracing::debug!("Collected data {:?}", frs);
                Ok(CollectFluidRegulatorsResponse { fluids: frs }.to_response())
            }
            Err(e) => {
                tracing::error!("There was an error collecting {}", e.to_string());
                Err(Status::cancelled(format!(
                    "Failed to query the database: {}",
                    e
                )))
            }
        }
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
        request: Request<AddInstructionRequest>,
    ) -> Result<Response<AddInstructionResponse>, Status> {
        tracing::debug!("Got request {:?}", request);
        let instruction = request
            .into_inner()
            .instruction
            .ok_or_else(|| Status::cancelled("Invalid request to add fluid regulator"))?;
        let query = instruction
            .gen_insert_query()
            .to_string(PostgresQueryBuilder);
        let input_result = self.connection.insert(query).await;
        match input_result {
            Ok(instruction_id) => {
                let instruction_response: Response<AddInstructionResponse> =
                    AddInstructionResponse { instruction_id }.to_response();
                Ok(instruction_response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to insert into database: {}",
                e
            ))),
        }
    }
    async fn remove_instruction(
        &self,
        request: Request<RemoveInstructionRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        tracing::debug!("Got Request {:?}", request);
        let instruction_id = request.into_inner().instruction_id;
        let query = Instruction::gen_remove_query(instruction_id).to_string(PostgresQueryBuilder);
        let delete_result = self.connection.delete(query).await;
        match delete_result {
            Ok(_) => {
                let remove_response = GenericRemovalResponse {}.to_response();
                Ok(remove_response)
            }
            Err(e) => Err(Status::aborted(e.to_string())),
        }
    }
    async fn collect_instructions(
        &self,
        request: Request<CollectInstructionRequest>,
    ) -> Result<Response<CollectInstructionResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let exprs = request
            .into_inner()
            .get_expressions()
            .map_err(|e| Status::cancelled(e.to_string()))?;
        let query = Instruction::gen_select_query_on_fields(InstructionSchema::Table, exprs)
            .to_string(PostgresQueryBuilder);
        let results = self.connection.select(query).await;
        match results {
            Ok(results) => {
                let instructions: Vec<Instruction> = results
                    .into_iter()
                    .map(|row| Instruction::try_from(row).unwrap())
                    .collect_vec();
                tracing::info!("Successfully collected instructions");
                tracing::debug!("Collected data {:?}", instructions);
                Ok(CollectInstructionResponse { instructions }.to_response())
            }
            Err(e) => {
                tracing::error!("There was an error collecting {}", e.to_string());
                Err(Status::cancelled(format!(
                    "Failed to query the database: {}",
                    e
                )))
            }
        }
    }
    async fn update_instruction(
        &self,
        request: Request<ModifyInstructionRequest>,
    ) -> Result<Response<ModifyInstructionResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let instruction = request
            .into_inner()
            .instruction
            .ok_or_else(|| Status::cancelled("Invalid request to remove instruction"))?;
        let query = instruction
            .gen_update_query()
            .to_string(PostgresQueryBuilder);
        let result = self.connection.update(query).await;
        match result {
            Ok(id) => {
                let response = ModifyInstructionResponse { instruction_id: id }.to_response();
                Ok(response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to update into database: {}",
                e
            ))),
        }
    }
    async fn add_ingredient(
        &self,
        request: Request<AddIngredientRequest>,
    ) -> Result<Response<AddIngredientResponse>, Status> {
        tracing::debug!("Got request {:?}", request);
        let ingredient = request
            .into_inner()
            .ingredient
            .ok_or_else(|| Status::cancelled("Invalid request to add ingredient"))?;
        let query = ingredient
            .gen_insert_query()
            .to_string(PostgresQueryBuilder);
        let input_result = self.connection.insert(query).await;
        match input_result {
            Ok(ingredient_id) => {
                let ingredient_response: Response<AddIngredientResponse> =
                    AddIngredientResponse { ingredient_id }.to_response();
                Ok(ingredient_response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to insert into database: {}",
                e
            ))),
        }
    }
    async fn remove_ingredient(
        &self,
        request: Request<RemoveIngredientRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        tracing::debug!("Got Request {:?}", request);
        let fr_id = request.into_inner().ingredient_id;
        let query = Ingredient::gen_remove_query(fr_id).to_string(PostgresQueryBuilder);
        let delete_result = self.connection.delete(query).await;
        match delete_result {
            Ok(_) => {
                let remove_response = GenericRemovalResponse {}.to_response();
                Ok(remove_response)
            }
            Err(e) => Err(Status::aborted(e.to_string())),
        }
    }
    async fn update_ingredient(
        &self,
        request: Request<ModifyIngredientRequest>,
    ) -> Result<Response<ModifyIngredientResponse>, Status> {
        tracing::debug!("Got Request {:?}", request);
        let ingredient = request
            .into_inner()
            .ingredient
            .ok_or_else(|| Status::cancelled("Invalid request to remove instruction"))?;
        let query = ingredient
            .gen_update_query()
            .to_string(PostgresQueryBuilder);
        let ingredient_update_result = self.connection.update(query).await;
        match ingredient_update_result {
            Ok(ingredient_id) => {
                // if request.update_fr {
                //     if let Some(fr) = ingredient.regulator {
                //         let request = ModifyFluidRegulatorRequest { fluid: Some(fr) };
                //         let _ = self.update_fluid_regulator(request.into_request()).await?;
                //     }
                // }
                // if request.update_instruction {
                //     if let Some(instruction) = ingredient.instruction {
                //         let request = ModifyInstructionRequest {
                //             instruction: Some(instruction),
                //         };
                //         let _ = self.update_instruction(request.into_request()).await?;
                //     }
                // }

                Ok(ModifyIngredientResponse { ingredient_id }.to_response())
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to update into database: {}",
                e
            ))),
        }
    }
    async fn collect_ingredients(
        &self,
        request: Request<CollectIngredientRequest>,
    ) -> Result<Response<CollectIngredientResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let exprs = request
            .into_inner()
            .get_expressions()
            .map_err(|e| Status::cancelled(e.to_string()))?;
        let query = Ingredient::gen_select_query_on_fields(IngredientSchema::Table, exprs)
            .to_string(PostgresQueryBuilder);
        let results = self.connection.select(query).await;
        match results {
            Ok(results) => {
                let ingredients: Vec<Ingredient> = results
                    .into_iter()
                    .map(|row| Ingredient::try_from(row).unwrap())
                    .collect_vec();
                // TDOO(TheFinalJoke): Refactor this to run on single query
                // TODO(TheFinalJoke): Refactor this so we are not sending another request
                let rebuilt_data = stream::iter(ingredients)
                    .then(|mut ingredient| async {
                        if let Some(fr_id) = ingredient.regulator.as_ref().and_then(|fr| fr.fr_id) {
                            ingredient.regulator =
                                self.parse_and_collect_fluid_regulator(fr_id).await;
                        }
                        if let Some(instruction_id) = ingredient
                            .instruction
                            .as_ref()
                            .map(|instruction| instruction.id)
                        {
                            ingredient.instruction =
                                self.parse_and_collect_instruction(instruction_id).await;
                        }
                        ingredient
                    })
                    .collect()
                    .await;
                tracing::info!("Successfully collected fluid regulators");
                tracing::debug!("Collected data {:?}", rebuilt_data);
                Ok(CollectIngredientResponse {
                    ingredients: rebuilt_data,
                }
                .to_response())
            }
            Err(e) => {
                tracing::error!("There was an error collecting {}", e.to_string());
                Err(Status::cancelled(format!(
                    "Failed to query the database: {}",
                    e
                )))
            }
        }
    }
    async fn reset_db(
        &self,
        request: Request<ResetRequest>,
    ) -> Result<Response<ResetResponse>, Status> {
        tracing::info!("Got request {:?}", request);
        let dropped_result = self.connection.truncate_schema().await;
        tracing::info!("the dropped Result {:?}", &dropped_result);
        match dropped_result {
            Ok(_) => {
                tracing::info!("Successfully dropped rows");
                Ok(ResetResponse {}.to_response())
            }
            Err(err) => Err(Status::cancelled(format!("Failed to drop rows: {}", err))),
        }
    }
}

impl DaemonServerContext {
    async fn parse_and_collect_fluid_regulator(&self, fr_id: i32) -> Option<FluidRegulator> {
        let req = CollectFluidRegulatorsRequest {
            expressions: vec![FetchData {
                column: "fr_id".to_string(),
                operation: Operation::Equal.into(),
                values: fr_id.to_string(),
            }],
        };
        match self.collect_fluid_regulators(req.into_request()).await {
            Ok(response) => response.into_inner().fluids.first().cloned(),
            Err(e) => {
                tracing::error!("Error Occured: {}", e);
                None
            }
        }
    }
    async fn parse_and_collect_instruction(&self, instruction_id: i32) -> Option<Instruction> {
        let req = CollectInstructionRequest {
            expressions: vec![FetchData {
                column: "instruction_id".to_string(),
                operation: Operation::Equal.into(),
                values: instruction_id.to_string(),
            }],
        };
        match self.collect_instructions(req.into_request()).await {
            Ok(response) => response.into_inner().instructions.first().cloned(),
            Err(e) => {
                tracing::error!("Error Occured: {}", e);
                None
            }
        }
    }
}

pub async fn start_server(
    service: UdmServiceServer<DaemonServerContext>,
    addr: SocketAddr,
) -> UdmResult<()> {
    tracing::info!("Running Udm Service on {:?}", &addr);
    let _ = Server::builder().add_service(service).serve(addr).await;
    Ok(())
}
