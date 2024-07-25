use crate::db::executor::GenQueries;
use crate::db::DbConnection;
use crate::db::DbMetaData;
use crate::db::DbType;
use crate::db::FluidRegulationSchema;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::db::InstructionToRecipeSchema;
use crate::db::RecipeSchema;
use crate::parsers::settings::UdmConfigurer;
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
use crate::rpc_types::service_types::AddRecipeInstOrderRequest;
use crate::rpc_types::service_types::AddRecipeInstOrderResponse;
use crate::rpc_types::service_types::AddRecipeRequest;
use crate::rpc_types::service_types::AddRecipeResponse;
use crate::rpc_types::service_types::CollectExpressions;
use crate::rpc_types::service_types::CollectFluidRegulatorsRequest;
use crate::rpc_types::service_types::CollectFluidRegulatorsResponse;
use crate::rpc_types::service_types::CollectIngredientRequest;
use crate::rpc_types::service_types::CollectIngredientResponse;
use crate::rpc_types::service_types::CollectInstructionRequest;
use crate::rpc_types::service_types::CollectInstructionResponse;
use crate::rpc_types::service_types::CollectRecipeInstOrderRequest;
use crate::rpc_types::service_types::CollectRecipeInstOrderResponse;
use crate::rpc_types::service_types::CollectRecipeRequest;
use crate::rpc_types::service_types::CollectRecipeResponse;
use crate::rpc_types::service_types::FetchData;
use crate::rpc_types::service_types::GenericEmpty;
use crate::rpc_types::service_types::GenericRemovalResponse;
use crate::rpc_types::service_types::InstructionToRecipeMetadata;
use crate::rpc_types::service_types::ModifyFluidRegulatorRequest;
use crate::rpc_types::service_types::ModifyFluidRegulatorResponse;
use crate::rpc_types::service_types::ModifyIngredientRequest;
use crate::rpc_types::service_types::ModifyIngredientResponse;
use crate::rpc_types::service_types::ModifyInstructionRequest;
use crate::rpc_types::service_types::ModifyInstructionResponse;
use crate::rpc_types::service_types::ModifyRecipeRequest;
use crate::rpc_types::service_types::ModifyRecipeResponse;
use crate::rpc_types::service_types::Operation;
use crate::rpc_types::service_types::RecipeInstructionOrder;
use crate::rpc_types::service_types::RemoveFluidRegulatorRequest;
use crate::rpc_types::service_types::RemoveIngredientRequest;
use crate::rpc_types::service_types::RemoveInstructionRequest;
use crate::rpc_types::service_types::RemoveRecipeInstOrderRequest;
use crate::rpc_types::service_types::RemoveRecipeRequest;
use crate::rpc_types::service_types::ResetRequest;
use crate::rpc_types::service_types::ResetResponse;
use crate::rpc_types::service_types::ServiceResponse;
use crate::rpc_types::service_types::UpdateRecipeInstOrderRequest;
use crate::rpc_types::Recipe;
use crate::UdmResult;
use anyhow::Result;
use futures::stream;
use futures::stream::StreamExt;
use itertools::Itertools;
use sea_query::PostgresQueryBuilder;
use signal_hook_tokio::SignalsInfo;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::IntoRequest;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tracing;

tonic::include_proto!("server");

pub trait GrpcServerFactory {}
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
        tracing::debug!("Got request {request:?}");
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
        tracing::debug!("Got Request {request:?}");
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
        request: Request<AddRecipeRequest>,
    ) -> Result<Response<AddRecipeResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let recipe = request
            .get_ref()
            .clone()
            .recipe
            .ok_or_else(|| Status::cancelled("Invalid request to add recipe"))?;
        let query = recipe.gen_insert_query().to_string(PostgresQueryBuilder);
        let response = self.connection.insert(query).await;
        match response {
            Ok(recipe_id) => {
                // Insert instruction order into db
                for (position, instruction) in recipe.instructions {
                    let order = InstructionToRecipeMetadata {
                        id: None,
                        recipe_id,
                        instruction_id: instruction.id,
                        instruction_order: position,
                    };
                    let order_query = order.gen_insert_query().to_string(PostgresQueryBuilder);
                    self.connection.insert(order_query).await.map_err(|e| {
                        let message = format!("Failed to query the database: {}", e);
                        tracing::error!(message);
                        Status::cancelled(message)
                    })?;
                }
                let response = AddRecipeResponse { recipe_id }.to_response();
                Ok(response)
            }
            Err(e) => {
                format!("Failed to insert into database: {}", e);
                Err(Status::data_loss(format!(
                    "Failed to insert into database: {}",
                    e
                )))
            }
        }
    }
    async fn remove_recipe(
        &self,
        request: Request<RemoveRecipeRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        tracing::debug!("Got Request {request:?}");
        let recipe_id = request.get_ref().recipe_id;
        let query = FluidRegulator::gen_remove_query(recipe_id).to_string(PostgresQueryBuilder);
        let delete_result = self.connection.delete(query).await;
        match delete_result {
            Ok(_) => {
                let remove_response = GenericRemovalResponse {}.to_response();
                Ok(remove_response)
            }
            Err(e) => Err(Status::aborted(e.to_string())),
        }
    }
    async fn update_recipe(
        &self,
        request: Request<ModifyRecipeRequest>,
    ) -> Result<Response<ModifyRecipeResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let recipe = request
            .get_ref()
            .clone()
            .recipe
            .ok_or_else(|| Status::cancelled("Invalid request to add recipe"))?;
        let query = recipe.gen_update_query().to_string(PostgresQueryBuilder);
        let response = self.connection.insert(query).await;
        match response {
            Ok(recipe_id) => {
                // Insert instruction order into db
                for (position, instruction) in recipe.instructions {
                    let order = InstructionToRecipeMetadata {
                        id: None,
                        recipe_id,
                        instruction_id: instruction.id,
                        instruction_order: position,
                    };
                    let order_query = order.gen_insert_query().to_string(PostgresQueryBuilder);
                    self.connection.update(order_query).await.map_err(|e| {
                        Status::cancelled(format!("Failed to query the database: {}", e))
                    })?;
                }
                let response = ModifyRecipeResponse { recipe_id }.to_response();
                Ok(response)
            }
            Err(e) => Err(Status::data_loss(format!(
                "Failed to update into database: {}",
                e
            ))),
        }
    }

    async fn collect_recipe(
        &self,
        request: Request<CollectRecipeRequest>,
    ) -> Result<Response<CollectRecipeResponse>, Status> {
        tracing::debug!("Got {:?}", request);
        let exprs = request
            .get_ref()
            .get_expressions()
            .map_err(|e| Status::cancelled(e.to_string()))?;
        let query = Recipe::gen_select_query_on_fields(RecipeSchema::Table, exprs)
            .to_string(PostgresQueryBuilder);
        let results = self.connection.select(query).await;
        match results {
            Ok(results) => {
                let recipes: Vec<Recipe> = results
                    .into_iter()
                    .map(|row| Recipe::try_from(row).unwrap())
                    .collect_vec();
                let rebuilt_data: Vec<Recipe> = stream::iter(recipes)
                    .then(|mut recipe| async {
                        // Collection instruction
                        let sorted = self
                            .parse_and_collect_instructions_to_recipe_by_recipe_id(recipe.id)
                            .await;
                        for instruct in sorted {
                            let instr = self
                                .parse_and_collect_instruction(instruct.instruction_id)
                                .await;
                            if let Some(ins) = instr {
                                recipe.instructions.insert(instruct.instruction_order, ins);
                            }
                        }
                        recipe
                    })
                    .collect()
                    .await;
                tracing::info!("Successfully collected instructions");
                tracing::debug!("Collected data {:?}", rebuilt_data);
                Ok(CollectRecipeResponse {
                    recipes: rebuilt_data,
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
    async fn add_instruction(
        &self,
        request: Request<AddInstructionRequest>,
    ) -> Result<Response<AddInstructionResponse>, Status> {
        tracing::debug!("Got request {request:?}");
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
        tracing::debug!("Got Request {request:?}");
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
        tracing::debug!("Got request {request:?}");
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
        tracing::debug!("Got Request {request:?}");
        let ingredient_id = request.into_inner().ingredient_id;
        let query = Ingredient::gen_remove_query(ingredient_id).to_string(PostgresQueryBuilder);
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
        tracing::debug!("Got Request {request:?}");
        let ingredient = request
            .get_ref()
            .clone()
            .ingredient
            .ok_or_else(|| Status::cancelled("Invalid request to remove instruction"))?;
        let query = ingredient
            .gen_update_query()
            .to_string(PostgresQueryBuilder);
        let ingredient_update_result = self.connection.update(query).await;
        match ingredient_update_result {
            Ok(ingredient_id) => {
                if request.get_ref().update_fr {
                    if let Some(fr) = ingredient.regulator {
                        let request = ModifyFluidRegulatorRequest { fluid: Some(fr) };
                        let _ = self.update_fluid_regulator(request.into_request()).await?;
                    }
                }
                if request.get_ref().update_instruction {
                    if let Some(instruction) = ingredient.instruction {
                        let request = ModifyInstructionRequest {
                            instruction: Some(instruction),
                        };
                        let _ = self.update_instruction(request.into_request()).await?;
                    }
                }

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
        tracing::info!("Got request {request:?}");
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
    async fn update_recipe_instruction_order(
        &self,
        request: Request<UpdateRecipeInstOrderRequest>,
    ) -> Result<Response<GenericEmpty>, Status> {
        tracing::debug!("Got {:?}", request);
        let order_requests: Vec<Option<InstructionToRecipeMetadata>> = request
            .get_ref()
            .recipe_orders
            .clone()
            .into_iter()
            .map(|req| InstructionToRecipeMetadata::try_from(req).ok())
            .collect_vec();
        let filtered_requests: HashSet<InstructionToRecipeMetadata> =
            order_requests.into_iter().flatten().collect();
        for req in filtered_requests {
            let query = req.gen_update_query().to_string(PostgresQueryBuilder);
            let result = self.connection.update(query).await;
            match result {
                Ok(id) => tracing::info!("Updated Recipe to Instruction Collection {}", id),
                Err(e) => {
                    tracing::error!("Error while updating the Recipe to Instruction {}", e);
                    return Err(Status::cancelled(
                        "Error while updating the Recipe to Instruction {e:?}",
                    ));
                }
            }
        }
        Ok(GenericEmpty {}.to_response())
    }
    async fn add_recipe_instruction_order(
        &self,
        request: Request<AddRecipeInstOrderRequest>,
    ) -> Result<Response<AddRecipeInstOrderResponse>, Status> {
        tracing::debug!("Got Request {request:?}");
        let recipe_orders = request.into_inner().recipe_orders;
        let ids = stream::iter(recipe_orders)
            .filter_map(|orders| async { InstructionToRecipeMetadata::try_from(orders).ok() })
            .then(|order| async move {
                let query = order.gen_insert_query().to_string(PostgresQueryBuilder);
                match self.connection.insert(query).await {
                    Ok(id) => {
                        tracing::info!("Successfully inserted into db {}", id);
                        Some(id)
                    }
                    Err(e) => {
                        tracing::error!("Error inserting into db {e:?}");
                        None
                    }
                }
            })
            .filter_map(|id| async move { id })
            .collect()
            .await;
        Ok(AddRecipeInstOrderResponse { ids }.to_response())
    }

    async fn collect_recipe_instruction_order(
        &self,
        request: Request<CollectRecipeInstOrderRequest>,
    ) -> Result<Response<CollectRecipeInstOrderResponse>, Status> {
        tracing::debug!("Got request {request:?}");
        let exprs = request
            .into_inner()
            .get_expressions()
            .map_err(|e| Status::cancelled(e.to_string()))?;
        let query = InstructionToRecipeMetadata::gen_select_query_on_fields(
            InstructionToRecipeSchema::Table,
            exprs,
        )
        .to_string(PostgresQueryBuilder);
        let results = self.connection.select(query).await;
        match results {
            Ok(results) => {
                let recipe_instrs: Vec<InstructionToRecipeMetadata> = results
                    .into_iter()
                    .map(|row| InstructionToRecipeMetadata::try_from(row).unwrap())
                    .collect_vec();
                tracing::info!("Successfully collected Instructions to Recipe");
                tracing::debug!("Collected data {:?}", recipe_instrs);
                let recipe_to_instructions: Vec<RecipeInstructionOrder> = recipe_instrs
                    .into_iter()
                    .map(|orders| orders.try_into().ok().unwrap())
                    .collect_vec();
                Ok(CollectRecipeInstOrderResponse {
                    recipe_to_instructions,
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
    async fn remove_recipe_instruction_order(
        &self,
        request: Request<RemoveRecipeInstOrderRequest>,
    ) -> Result<Response<GenericRemovalResponse>, Status> {
        tracing::debug!("Got Request {request:?}");
        let recipe_inst_id = request.into_inner().id;
        let query = InstructionToRecipeMetadata::gen_remove_query(recipe_inst_id)
            .to_string(PostgresQueryBuilder);
        let delete_result = self.connection.delete(query).await;
        match delete_result {
            Ok(_) => {
                let remove_response = GenericRemovalResponse {}.to_response();
                Ok(remove_response)
            }
            Err(e) => Err(Status::aborted(e.to_string())),
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
    async fn parse_and_collect_instructions_to_recipe_by_recipe_id(
        &self,
        recipe_id: i32,
    ) -> Vec<InstructionToRecipeMetadata> {
        // Collection instruction
        let fetch_data = vec![FetchData {
            column: "recipe_id".to_string(),
            operation: Operation::Equal.into(),
            values: recipe_id.to_string(),
        }
        .to_simple_expr(RecipeSchema::RecipeId)
        .unwrap()];
        let data_query = InstructionToRecipeMetadata::gen_select_query_on_fields(
            InstructionToRecipeSchema::Table,
            fetch_data,
        )
        .to_string(PostgresQueryBuilder);
        let ordered_data = self.connection.select(data_query).await;
        match ordered_data {
            Ok(data) => {
                let meta: Vec<_> = data
                    .into_iter()
                    .map(|row| InstructionToRecipeMetadata::try_from(row).unwrap())
                    .collect();
                let sorted: Vec<_> = meta
                    .into_iter()
                    .sorted_by_key(|row| row.instruction_order)
                    .collect();
                sorted
            }
            Err(err) => {
                tracing::error!("{}", err.to_string());
                Vec::new()
            }
        }
    }
    // Built this but do not need it anymore, but might be useful later
    #[allow(dead_code)]
    async fn parse_and_collect_instructions_to_recipe_by_id(
        &self,
        ids: Vec<i32>,
    ) -> Vec<InstructionToRecipeMetadata> {
        // Collection instruction
        let fetch_data = vec![FetchData {
            column: "id".to_string(),
            operation: Operation::In.into(),
            values: format!("{:?}", ids),
        }
        .to_simple_expr(InstructionToRecipeSchema::Id)
        .unwrap()];
        let data_query = InstructionToRecipeMetadata::gen_select_query_on_fields(
            InstructionToRecipeSchema::Table,
            fetch_data,
        )
        .to_string(PostgresQueryBuilder);
        let ordered_data = self.connection.select(data_query).await;
        match ordered_data {
            Ok(data) => {
                let meta: Vec<_> = data
                    .into_iter()
                    .map(|row| InstructionToRecipeMetadata::try_from(row).unwrap())
                    .collect();
                let sorted: Vec<_> = meta
                    .into_iter()
                    .sorted_by_key(|row| row.instruction_order)
                    .collect();
                sorted
            }
            Err(err) => {
                tracing::error!("{}", err.to_string());
                Vec::new()
            }
        }
    }
}
pub struct SqlDaemonServer {
    configuration: Arc<UdmConfigurer>,
    addr: SocketAddr,
}
impl SqlDaemonServer {
    pub fn new(config: Arc<UdmConfigurer>, addr: SocketAddr) -> Self {
        Self {
            configuration: config,
            addr,
        }
    }
    async fn build_context(&self) -> DaemonServerContext {
        let db_type = Arc::new(DbType::load_db(Arc::clone(&self.configuration)));
        let mut connection = db_type.establish_connection().await;
        tracing::info!("Initializing database");
        let _ = connection
            .gen_schmea()
            .await
            .map_err(|e| format!("Failed to create database schema {}", e));
        tracing::info!("Attempting to Udm Sql Daemon Service on {}", self.addr);
        let db_metadata = DbMetaData::new(Arc::clone(&db_type));
        DaemonServerContext::new(connection, self.addr, db_metadata)
    }
    pub async fn start_server(&self) -> UdmResult<()> {
        let daemon_server = self.build_context().await;
        let udm_service = UdmServiceServer::new(daemon_server);
        tracing::info!("Running Udm Sql Daemon Service on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(udm_service)
            .serve(self.addr)
            .await;
        Ok(())
    }
    pub async fn start_server_with_signal(&self, mut signal: SignalsInfo) -> UdmResult<()> {
        let daemon_server = self.build_context().await;
        let udm_service = UdmServiceServer::new(daemon_server);
        tracing::info!("Running Udm Sql Daemon Service on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(udm_service)
            .serve_with_shutdown(self.addr, async {
                let _ = signal.next().await;
                tracing::info!("Got a termination signal");
            })
            .await;
        Ok(())
    }
}
impl GrpcServerFactory for SqlDaemonServer {}
