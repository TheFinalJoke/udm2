use std::collections::HashMap;

use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::UdmServerOptions;
use clap::Args;
use clap::Subcommand;
use lib::error::UdmError;
use lib::rpc_types::recipe_types::DrinkSize;
use lib::rpc_types::recipe_types::Recipe;
use lib::rpc_types::service_types::AddRecipeRequest;
use lib::rpc_types::MultipleValues;
use lib::UdmResult;
use tonic::async_trait;

use super::helpers::UdmGrpcActions;

#[derive(Subcommand, Debug)]
pub enum RecipeCommands {
    #[command(about = "Add a Recipe")]
    Add(AddRecipeArgs),
    #[command(about = "Show current Recipes")]
    Show(ShowRecipeArgs),
    #[command(about = "Remove current a recipe")]
    Remove(RemoveRecipeArgs),
    #[command(about = "Update a recipe")]
    Update(UpdateRecipeArgs),
}
#[async_trait]
impl MainCommandHandler for RecipeCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            RecipeCommands::Add(user_input) => user_input.handle_command(options).await,
            RecipeCommands::Show(user_input) => todo!(),
            RecipeCommands::Remove(user_input) => todo!(),
            RecipeCommands::Update(user_input) => todo!(),
        }
    }
}
#[derive(Args, Debug)]
pub struct AddRecipeArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        default_value = "",
        exclusive = true
    )]
    raw: String,
    #[arg(
        short = 'n',
        long,
        help = "Name of the Recipe",
        default_value = "",
        required_unless_present = "raw"
    )]
    name: String,
    #[arg(short = 's', long, help="Size of the drink", value_parser=DrinkSize::get_possible_values(), default_value = DrinkSize::Unspecified.as_str_name(), required_unless_present="raw")]
    size: String,
    #[arg(
        short = 'd',
        long,
        help = "Description of the recipe",
        default_value = "",
        required_unless_present = "raw"
    )]
    description: String,
}
impl UdmGrpcActions<Recipe> for AddRecipeArgs {
    fn sanatize_input(&self) -> UdmResult<Recipe> {
        if !&self.raw.is_empty() {
            tracing::debug!("Json passed: {}", &self.raw);
            let recipe: Recipe = serde_json::from_str(&self.raw)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            return Ok(recipe);
        }

        if self.name.is_empty() || self.description.is_empty() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(Recipe {
            id: 0,
            name: self.name.clone(),
            size: DrinkSize::from_str_name(&self.size)
                .unwrap_or(DrinkSize::Unspecified)
                .into(),
            instructions: HashMap::new(),
            user_input: true,
            description: self.description.clone(),
        })
    }
}
#[async_trait]
impl MainCommandHandler for AddRecipeArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let recipe = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_recipe(AddRecipeRequest {
                recipe: Some(recipe),
            })
            .await
            .map_err(|e| {
                tracing::error!("Error occured: {:?}", e);
                UdmError::ApiFailure(format!("{}", e))
            })?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got ID back {}",
            response.get_ref().recipe_id
        );
        Ok(())
    }
}
#[derive(Args, Debug)]
pub struct ShowRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Args, Debug)]
pub struct RemoveRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}
#[derive(Args, Debug)]
pub struct UpdateRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}
