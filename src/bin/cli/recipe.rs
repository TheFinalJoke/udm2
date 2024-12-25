use crate::cli::helpers::ensure_removal;
use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::ShowHandler;
use crate::cli::helpers::UdmServerOptions;
use clap::Args;
use clap::Subcommand;
use cli_table::Cell;
use cli_table::Style;
use cli_table::Table;
use cli_table::TableStruct;
use lib::db::RecipeSchema;
use lib::error::trace_log_error;
use lib::error::UdmError;
use lib::rpc_types::recipe_types::DrinkSize;
use lib::rpc_types::recipe_types::Recipe;
use lib::rpc_types::service_types::AddRecipeRequest;
use lib::rpc_types::service_types::CollectRecipeRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::ModifyRecipeRequest;
use lib::rpc_types::service_types::RemoveRecipeRequest;
use lib::rpc_types::FieldValidation;
use lib::rpc_types::MultipleValues;
use lib::UdmResult;
use std::collections::HashMap;
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
            RecipeCommands::Show(user_input) => user_input.handle_command(options).await,
            RecipeCommands::Remove(user_input) => user_input.handle_command(options).await,
            RecipeCommands::Update(user_input) => user_input.handle_command(options).await,
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
            let recipe: Recipe = serde_json::from_str(&self.raw).map_err(|_| {
                trace_log_error(UdmError::InvalidInput(String::from("Failed to parse json")))
            })?;
            return Ok(recipe);
        }

        if self.name.is_empty() || self.description.is_empty() {
            return Err(trace_log_error(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            ))));
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
        let mut open_connection = options.connect_to_udm().await?;
        let response = open_connection
            .add_recipe(AddRecipeRequest {
                recipe: Some(recipe),
            })
            .await
            .map_err(|e| {
                tracing::error!("Error occured: {:?}", e);
                trace_log_error(UdmError::ApiFailure(format!("{}", e)))
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
    query_options: Option<String>,
    #[arg(long, short = 'e', help = "Example queries", default_value = "false")]
    example: bool,
    #[arg(long, short = 's', help = "show_fields", default_value = "false")]
    show_fields: bool,
}

#[async_trait]
impl MainCommandHandler for ShowRecipeArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if self.example {
            ShowRecipeArgs::show_example();
            Ok(())
        } else if self.show_fields {
            Self::get_schema_columns();
            Ok(())
        } else {
            let fetched = match self.sanatize_input() {
                Ok(fetch) => fetch,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1)
                }
            };
            let mut open_connection = options.connect_to_udm().await?;
            let response = open_connection
                .collect_recipe(CollectRecipeRequest {
                    expressions: fetched,
                })
                .await
                .map_err(|e| trace_log_error(UdmError::ApiFailure(format!("{}", e))));
            match response {
                Ok(response) => {
                    tracing::debug!("Got response {:?}", &response);
                    let recipes = &response.get_ref().recipes;
                    println!("Found {} results", &recipes.len());
                    let table = self.create_tables(recipes.to_vec());
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
impl ShowHandler<Recipe> for ShowRecipeArgs {
    fn show_example() {
        println!("To build a query it will be <field><operation><values>");
        println!("fr_id=1");
        println!("^^ will query fr_id=1");
        println!("Another example of multiple values, fr_id = 1");
        Self::get_schema_columns();
    }

    fn create_tables(&self, data: Vec<Recipe>) -> TableStruct {
        let mut table = Vec::new();
        for recipe in data {
            table.push(vec![
                recipe.id.cell(),
                recipe.name.cell(),
                DrinkSize::try_from(recipe.size)
                    .unwrap_or(DrinkSize::Unspecified)
                    .cell(),
                recipe.user_input.cell(),
                recipe.description.cell(),
            ]);
        }
        table
            .table()
            .title(vec![
                "ID".cell().bold(true),
                "Name".cell().bold(true),
                "Drink Size".cell().bold(true),
                "Inputed from User".cell().bold(true),
                "Description".cell().bold(true),
            ])
            .bold(true)
    }

    fn get_schema_columns() {
        println!("{}", RecipeSchema::RecipeId);
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
pub struct RemoveRecipeArgs {
    #[arg(short, long, help = "Recipe id to remove", required = true)]
    recipe_id: Option<i32>,
    #[arg(
        short,
        long,
        help = "Does not prompt, you are absolutely sure",
        default_value = "false"
    )]
    yes: bool,
}
#[async_trait]
impl MainCommandHandler for RemoveRecipeArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let id = self.recipe_id.ok_or_else(|| {
            trace_log_error(UdmError::InvalidInput(
                "Invalid input to remove fluid regulator".to_string(),
            ))
        })?;
        if !self.yes {
            let _ = ensure_removal();
        }
        let req = RemoveRecipeRequest { recipe_id: id };
        let mut open_conn = options.connect_to_udm().await?;
        let response = open_conn.remove_recipe(req).await;
        tracing::debug!("Got response {:?}", response);
        match response {
            Ok(_) => {
                tracing::info!("Successfully removed from database");
            }
            Err(err) => {
                tracing::error!("Error removing from db: {}", err.to_string())
            }
        }
        Ok(())
    }
}
#[derive(Args, Debug)]
pub struct UpdateRecipeArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        default_value = "",
        exclusive = true
    )]
    raw: String,
    #[arg(
        short = 'i',
        long,
        help = "ID you want to update",
        required_unless_present = "raw"
    )]
    recipe_id: i32,
    #[arg(short = 'n', long, help = "Name of the Recipe", default_value = "")]
    name: Option<String>,
    #[arg(short = 's', long, help="Size of the drink", value_parser=DrinkSize::get_possible_values(), required_unless_present="raw")]
    size: Option<String>,
    #[arg(
        short = 'd',
        long,
        help = "Description of the recipe",
        default_value = ""
    )]
    description: Option<String>,
}
#[async_trait]
impl MainCommandHandler for UpdateRecipeArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let recipe = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect_to_udm().await?;
        let response = open_connection
            .update_recipe(ModifyRecipeRequest {
                recipe: Some(recipe),
            })
            .await
            .map_err(|e| trace_log_error(UdmError::ApiFailure(format!("{}", e))))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got ID back {}",
            response.into_inner().recipe_id
        );
        Ok(())
    }
}
impl UdmGrpcActions<Recipe> for UpdateRecipeArgs {
    fn sanatize_input(&self) -> UdmResult<Recipe> {
        if !self.raw.is_empty() {
            tracing::debug!("Json passed: {}", &self.raw);
            let ingredient: Recipe = serde_json::from_str(&self.raw).map_err(|_| {
                trace_log_error(UdmError::InvalidInput(String::from("Failed to parse json")))
            })?;
            ingredient.validate_without_id_fields()?;
            return Ok(ingredient);
        }
        self.validate_all_fields()?;
        self.try_into()
    }
}
impl FieldValidation for UpdateRecipeArgs {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.recipe_id == 0 {
            return Err(trace_log_error(UdmError::InvalidInput(
                "Recipe ID is not set".to_string(),
            )));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        unimplemented!()
    }
}
impl TryFrom<&UpdateRecipeArgs> for Recipe {
    type Error = UdmError;

    fn try_from(value: &UpdateRecipeArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.recipe_id,
            name: value.name.clone().unwrap_or("".to_string()),
            size: DrinkSize::from_str_name(
                &value
                    .size
                    .clone()
                    .unwrap_or(DrinkSize::default().as_str_name().to_string()),
            )
            .unwrap()
            .into(),
            instructions: HashMap::new(),
            user_input: true,
            description: value.description.clone().unwrap_or("".to_string()),
        })
    }
}
