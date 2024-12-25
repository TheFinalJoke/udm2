use crate::cli::helpers::ensure_removal;
use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::ShowHandler;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::UdmServerOptions;
use async_trait::async_trait;
use clap::Args;
use clap::Subcommand;
use cli_table::Cell;
use cli_table::Style;
use cli_table::Table;
use cli_table::TableStruct;
use lib::db::InstructionToRecipeSchema;
use lib::error::trace_log_error;
use lib::error::UdmError;
use lib::rpc_types::service_types::AddRecipeInstOrderRequest;
use lib::rpc_types::service_types::CollectRecipeInstOrderRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::RecipeInstructionOrder;
use lib::rpc_types::service_types::RemoveRecipeInstOrderRequest;
use lib::rpc_types::service_types::UpdateRecipeInstOrderRequest;
use lib::rpc_types::FieldValidation;
use lib::UdmResult;

#[derive(Subcommand, Debug)]
pub enum RecipeToInstructionCommands {
    #[command(about = "Add a an individual instruction order")]
    Add(AddInstructionOrderArgs),
    #[command(about = "Show Instructions to recipe")]
    Show(ShowInstructionOrderArgs),
    #[command(about = "Remove instruction order")]
    Remove(RemoveInstructionOrderArgs),
    #[command(about = "Update the instruction order")]
    Update(UpdateInstructionOrderArgs),
}
#[async_trait]
impl MainCommandHandler for RecipeToInstructionCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            RecipeToInstructionCommands::Add(user_input) => {
                user_input.handle_command(options).await
            }
            RecipeToInstructionCommands::Show(user_input) => {
                user_input.handle_command(options).await
            }
            RecipeToInstructionCommands::Remove(user_input) => {
                user_input.handle_command(options).await
            }
            RecipeToInstructionCommands::Update(user_input) => {
                user_input.handle_command(options).await
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct UpdateInstructionOrderArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        default_value = "",
        exclusive = true
    )]
    raw: Option<String>,
    #[arg(
        short = 'd',
        long,
        help = "ID for entry",
        required_unless_present = "raw"
    )]
    id: Option<i32>,
    #[arg(short = 'r', long, help = "Recipe ID", required_unless_present = "raw")]
    recipe_id: Option<i32>,
    #[arg(
        short = 'i',
        long,
        help = "Instruction ID",
        required_unless_present = "raw"
    )]
    instruction_id: Option<i32>,
    #[arg(
        short = 'p',
        long,
        help = "Postition in the recipe",
        required_unless_present = "raw"
    )]
    position: Option<i32>,
}
#[async_trait]
impl MainCommandHandler for UpdateInstructionOrderArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let recipe_order = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect_to_udm().await?;
        let response = open_connection
            .update_recipe_instruction_order(UpdateRecipeInstOrderRequest {
                recipe_orders: [recipe_order].to_vec(),
                recipe_id: recipe_order.recipe_id,
            })
            .await
            .map_err(|e| trace_log_error(UdmError::ApiFailure(format!("{}", e))))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!("Updated into database");
        println!("Updated into database");
        Ok(())
    }
}
impl UdmGrpcActions<RecipeInstructionOrder> for UpdateInstructionOrderArgs {
    fn sanatize_input(&self) -> UdmResult<RecipeInstructionOrder> {
        if self.raw.is_some() && !self.raw.clone().unwrap().is_empty() {
            tracing::debug!("Json passed: {:?}", &self.raw);
            let recipe_order: RecipeInstructionOrder =
                serde_json::from_str(&self.raw.clone().unwrap()).map_err(|_| {
                    trace_log_error(UdmError::InvalidInput(String::from("Failed to parse json")))
                })?;
            if self.id.is_none() || self.id.unwrap() == 0 {
                return Err(trace_log_error(UdmError::InvalidInput(
                    "Ingredient ID is not set".to_string(),
                )));
            }
            return Ok(recipe_order);
        }
        self.validate_all_fields()?;
        self.try_into()
    }
}
impl FieldValidation for UpdateInstructionOrderArgs {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.id.is_none() || self.id.unwrap() == 0 {
            return Err(trace_log_error(UdmError::InvalidInput(
                "Ingredient ID is not set".to_string(),
            )));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        if self.id.is_none()
            || self.id.unwrap() == 0
            || self.instruction_id.is_none()
            || self.instruction_id.unwrap() == 0
        {
            return Err(trace_log_error(UdmError::InvalidInput(
                "Ingredient ID is not set".to_string(),
            )));
        }
        Ok(())
    }
}
impl TryFrom<&UpdateInstructionOrderArgs> for RecipeInstructionOrder {
    type Error = UdmError;

    fn try_from(value: &UpdateInstructionOrderArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            recipe_id: value.recipe_id.unwrap(),
            instruction_id: value.instruction_id.unwrap(),
            position: value.position.unwrap(),
        })
    }
}
#[derive(Args, Debug)]
pub struct AddInstructionOrderArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        exclusive = true
    )]
    raw: Option<String>,
    #[arg(short = 'r', long, help = "Recipe ID", required_unless_present = "raw")]
    recipe_id: Option<i32>,
    #[arg(
        short = 'i',
        long,
        help = "Instruction ID",
        required_unless_present = "raw"
    )]
    instruction_id: Option<i32>,
    #[arg(
        short = 'p',
        long,
        help = "Postition in the recipe",
        required_unless_present = "raw"
    )]
    position: Option<i32>,
}
#[async_trait]
impl MainCommandHandler for AddInstructionOrderArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let instruct_recipe = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect_to_udm().await?;
        let response = open_connection
            .add_recipe_instruction_order(AddRecipeInstOrderRequest {
                recipe_orders: [instruct_recipe].to_vec(),
            })
            .await
            .map_err(|e| {
                tracing::error!("{}", &e.to_string());
                trace_log_error(UdmError::ApiFailure(format!("{}", e)))
            })?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got IDs back {:?}",
            response.get_ref().ids
        );
        println!("Inserted into database: IDs {:?}", response.get_ref().ids);
        Ok(())
    }
}
impl TryFrom<&AddInstructionOrderArgs> for RecipeInstructionOrder {
    type Error = UdmError;

    fn try_from(value: &AddInstructionOrderArgs) -> Result<Self, Self::Error> {
        Ok(RecipeInstructionOrder {
            recipe_id: value.recipe_id.unwrap(),
            instruction_id: value.instruction_id.unwrap(),
            position: value.position.unwrap(),
            id: None,
        })
    }
}
impl UdmGrpcActions<RecipeInstructionOrder> for AddInstructionOrderArgs {
    fn sanatize_input(&self) -> UdmResult<RecipeInstructionOrder> {
        if self.raw.is_some() {
            tracing::debug!("Json passed: {}", &self.raw.clone().unwrap());
            let recipe_order: RecipeInstructionOrder =
                serde_json::from_str(&self.raw.clone().unwrap()).map_err(|_| {
                    trace_log_error(UdmError::InvalidInput(String::from("Failed to parse json")))
                })?;
            if self.recipe_id.is_none() || self.instruction_id.is_none() || self.position.is_none()
            {
                return Err(trace_log_error(UdmError::InvalidInput(
                    "Not all values are present".to_string(),
                )));
            }
            return Ok(recipe_order);
        }
        self.try_into()
    }
}

impl FieldValidation for AddInstructionOrderArgs {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.recipe_id.is_none() || self.instruction_id.is_none() || self.position.is_none() {
            return Err(trace_log_error(UdmError::InvalidInput(
                "Not all values are present".to_string(),
            )));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        unimplemented!()
    }
}
#[derive(Args, Debug)]
pub struct ShowInstructionOrderArgs {
    query_options: Option<String>,
    #[arg(long, short = 'e', help = "Example queries", default_value = "false")]
    example: bool,
    #[arg(long, short = 's', help = "show_fields", default_value = "false")]
    show_fields: bool,
}
#[async_trait]
impl MainCommandHandler for ShowInstructionOrderArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if self.example {
            Self::show_example();
            Ok(())
        } else if self.show_fields {
            Self::get_schema_columns();
            Ok(())
        } else {
            let fetched = self.sanatize_input()?;
            let mut open_connection = options.connect_to_udm().await?;
            let response = open_connection
                .collect_recipe_instruction_order(CollectRecipeInstOrderRequest {
                    expressions: fetched,
                })
                .await
                .map_err(|e| trace_log_error(UdmError::ApiFailure(format!("{}", e))));
            match response {
                Ok(response) => {
                    tracing::debug!("Got response {:?}", &response);
                    let recipe_orders = response.into_inner().recipe_to_instructions;
                    println!("Found {} results", &recipe_orders.len());
                    let table = self.create_tables(recipe_orders);
                    println!("{}", table.display().unwrap());
                    Ok(())
                }
                Err(err) => {
                    println!("Error: Could not show FRs due to: {}", err);
                    Ok(())
                }
            }
        }
    }
}
impl ShowHandler<RecipeInstructionOrder> for ShowInstructionOrderArgs {
    fn show_example() {
        println!("To build a query it will be <field><operation><values>");
        println!("fr_id=1");
        println!("^^ will query fr_id=1");
        println!("Another example of multiple values, fr_id = 1");
        Self::get_schema_columns();
    }

    fn create_tables(&self, data: Vec<RecipeInstructionOrder>) -> TableStruct {
        let mut table = Vec::new();
        for recipe_order in data {
            table.push(vec![
                recipe_order.id.unwrap_or(0).cell(),
                recipe_order.recipe_id.cell(),
                recipe_order.instruction_id.cell(),
                recipe_order.position.cell(),
            ]);
        }
        table
            .table()
            .title(vec![
                "ID".cell().bold(true),
                "Recipe ID".cell().bold(true),
                "Instruction ID".cell().bold(true),
                "Position".cell().bold(true),
            ])
            .bold(true)
    }

    fn get_schema_columns() {
        println!("{}", InstructionToRecipeSchema::Id);
    }
    fn sanatize_input(&self) -> UdmResult<Vec<FetchData>> {
        if self.query_options.is_none() {
            return Err(trace_log_error(UdmError::InvalidInput(
                "Error while parsing query".to_string(),
            )));
        }
        let collected_queries =
            FetchData::to_fetch_data_vec(self.query_options.clone().unwrap().as_str())?;
        Ok(collected_queries)
    }
}
#[derive(Args, Debug)]
pub struct RemoveInstructionOrderArgs {
    #[arg(short, long, required = true)]
    id: i32,
    #[arg(
        short,
        long,
        help = "Does not prompt, you are absolutely sure",
        default_value = "true"
    )]
    yes: bool,
}

#[async_trait]
impl MainCommandHandler for RemoveInstructionOrderArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if !self.yes {
            let _ = ensure_removal();
        }
        let req = RemoveRecipeInstOrderRequest { id: self.id };
        let mut open_conn = options.connect_to_udm().await?;
        let response = open_conn.remove_recipe_instruction_order(req).await;
        tracing::debug!("Got response {:?}", response);
        match response {
            Ok(_) => {
                tracing::info!("Successfully removed from database");
                println!("Successfully removed from database")
            }
            Err(err) => {
                tracing::error!("Error removing from db: {}", err.to_string())
            }
        }
        Ok(())
    }
}
