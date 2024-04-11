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
use lib::db::IngredientSchema;
use lib::error::UdmError;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::rpc_types::recipe_types::Ingredient;
use lib::rpc_types::recipe_types::IngredientType;
use lib::rpc_types::recipe_types::Instruction;
use lib::rpc_types::service_types::AddIngredientRequest;
use lib::rpc_types::service_types::CollectIngredientRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::ModifyIngredientRequest;
use lib::rpc_types::service_types::RemoveIngredientRequest;
use lib::rpc_types::FieldValidation;
use lib::rpc_types::MultipleValues;
use lib::UdmResult;

#[derive(Subcommand, Debug)]
pub enum IngredientCommands {
    #[command(about = "Add a Ingredient")]
    Add(AddIngredientArgs),
    #[command(about = "Show current Ingredient")]
    Show(ShowIngredientArgs),
    #[command(about = "Remove current Ingredient ")]
    Remove(RemoveIngredientArgs),
    #[command(about = "Update a fluid regulator")]
    Update(UpdateIngredientArgs),
}

#[async_trait]
impl MainCommandHandler for IngredientCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            IngredientCommands::Add(user_input) => user_input.handle_command(options).await,
            IngredientCommands::Show(user_input) => user_input.handle_command(options).await,
            IngredientCommands::Remove(user_input) => user_input.handle_command(options).await,
            IngredientCommands::Update(user_input) => user_input.handle_command(options).await,
        }
    }
}

#[derive(Args, Debug)]
pub struct AddIngredientArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        exclusive = true
    )]
    raw: String,
    #[arg(short, long, help = "Specify the ID")]
    ingredient_id: i32,
    #[arg(short, long, help = "Name of the ingreident")]
    name: String,
    #[arg(short, long, help = "If an ingredient in alcoholic")]
    is_alcoholic: bool,
    #[arg(short, long, help = "If an ingredient tied to a Fluid Device")]
    fr_id: i32,
    #[arg(short, long, help = "Amount of an ingredient")]
    amount: f32,
    #[arg(short, long, help = "Description of the Ingredient")]
    description: String,
    #[arg(short, long, help = "Type of ingredient", value_parser=IngredientType::get_possible_values())]
    ingredient_type: i32,
    #[arg(
        short,
        long,
        help = "The Instruction that goes along with the ingredient"
    )]
    instruction_id: Option<i32>,
    // is_active should be metadata and controlled by the application
}

#[async_trait]
impl MainCommandHandler for AddIngredientArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let ingredient = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_ingredient(AddIngredientRequest {
                ingredient: Some(ingredient),
            })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got ID back {}",
            response.into_inner().ingredient_id
        );
        Ok(())
    }
}
impl TryFrom<&AddIngredientArgs> for Ingredient {
    type Error = UdmError;

    fn try_from(value: &AddIngredientArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.ingredient_id,
            name: value.name.clone(),
            is_active: false,
            is_alcoholic: value.is_alcoholic,
            regulator: Some(FluidRegulator {
                fr_id: Some(value.fr_id),
                ..Default::default()
            }),
            amount: value.amount,
            description: value.description.clone(),
            ingredient_type: value.ingredient_type,
            instruction: {
                value.instruction_id.map(|id| Instruction {
                    id,
                    ..Default::default()
                })
            },
        })
    }
}
impl UdmGrpcActions<Ingredient> for AddIngredientArgs {
    fn sanatize_input(&self) -> UdmResult<Ingredient> {
        if !self.raw.is_empty() {
            tracing::debug!("Json passed: {}", &self.raw);
            let ingredient: Ingredient = serde_json::from_str(&self.raw)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            ingredient.validate_without_id_fields()?;
            return Ok(ingredient);
        }
        self.validate_all_fields()?;
        self.try_into()
    }
}
impl FieldValidation for AddIngredientArgs {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.ingredient_id == 0
            || self.name.is_empty()
            || self.description.is_empty()
            || self.ingredient_type == 0
        {
            return Err(UdmError::InvalidInput(
                "Not all values are present".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        if self.name.is_empty() || self.description.is_empty() || self.ingredient_type == 0 {
            return Err(UdmError::InvalidInput(
                "Not all values are present".to_string(),
            ));
        }
        Ok(())
    }
}
#[derive(Args, Debug)]
pub struct UpdateIngredientArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Raw json to transform",
        exclusive = true
    )]
    raw: String,
    #[arg(short, long, help = "Specify the ID")]
    ingredient_id: i32,
    #[arg(short, long, help = "Name of the ingreident")]
    name: String,
    #[arg(short, long, help = "If an ingredient in alcoholic")]
    is_alcoholic: bool,
    #[arg(short, long, help = "If an ingredient tied to a Fluid Device")]
    fr_id: i32,
    #[arg(short, long, help = "Amount of an ingredient")]
    amount: f32,
    #[arg(short, long, help = "Description of the Ingredient")]
    description: String,
    #[arg(short, long, help = "Type of ingredient", value_parser=IngredientType::get_possible_values())]
    ingredient_type: i32,
    #[arg(
        short,
        long,
        help = "The Instruction that goes along with the ingredient"
    )]
    instruction_id: Option<i32>,
}

#[async_trait]
impl MainCommandHandler for UpdateIngredientArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let ingredient = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .update_ingredient(ModifyIngredientRequest {
                ingredient: Some(ingredient),
            })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got ID back {}",
            response.into_inner().ingredient_id
        );
        Ok(())
    }
}
impl UdmGrpcActions<Ingredient> for UpdateIngredientArgs {
    fn sanatize_input(&self) -> UdmResult<Ingredient> {
        if !self.raw.is_empty() {
            tracing::debug!("Json passed: {}", &self.raw);
            let ingredient: Ingredient = serde_json::from_str(&self.raw)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            ingredient.validate_without_id_fields()?;
            return Ok(ingredient);
        }
        self.validate_all_fields()?;
        self.try_into()
    }
}
impl FieldValidation for UpdateIngredientArgs {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.ingredient_id == 0 {
            return Err(UdmError::InvalidInput(
                "Ingredient ID is not set".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        unimplemented!()
    }
}
impl TryFrom<&UpdateIngredientArgs> for Ingredient {
    type Error = UdmError;

    fn try_from(value: &UpdateIngredientArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.ingredient_id,
            name: value.name.clone(),
            is_active: false,
            is_alcoholic: value.is_alcoholic,
            regulator: Some(FluidRegulator {
                fr_id: Some(value.fr_id),
                ..Default::default()
            }),
            amount: value.amount,
            description: value.description.clone(),
            ingredient_type: value.ingredient_type,
            instruction: {
                value.instruction_id.map(|id| Instruction {
                    id,
                    ..Default::default()
                })
            },
        })
    }
}

#[derive(Args, Debug)]
pub struct ShowIngredientArgs {
    query_options: Option<String>,
    #[arg(long, short = 'e', help = "Example queries", default_value = "false")]
    example: bool,
    #[arg(long, short = 's', help = "show_fields", default_value = "false")]
    show_fields: bool,
}
#[async_trait]
impl MainCommandHandler for ShowIngredientArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if self.example {
            Self::show_example();
            Ok(())
        } else if self.show_fields {
            Self::get_schema_columns();
            Ok(())
        } else {
            let fetched = self.sanatize_input()?;
            let mut open_connection = options.connect().await?;
            let response = open_connection
                .collect_ingredients(CollectIngredientRequest {
                    expressions: fetched,
                })
                .await
                .map_err(|e| UdmError::ApiFailure(format!("{}", e)));
            match response {
                Ok(response) => {
                    tracing::debug!("Got response {:?}", &response);
                    let fluids = response.into_inner().ingredients;
                    println!("Found {} results", &fluids.len());
                    let table = self.create_tables(fluids);
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
impl ShowHandler<Ingredient> for ShowIngredientArgs {
    fn show_example() {
        println!("To build a query it will be <field><operation><values>");
        println!("fr_id=1");
        println!("^^ will query fr_id=1");
        println!("Another example of multiple values, fr_id = 1");
        Self::get_schema_columns();
    }

    fn create_tables(&self, data: Vec<Ingredient>) -> TableStruct {
        let mut table = Vec::new();
        for ingredient in data {
            table.push(vec![
                ingredient.id.cell(),
                ingredient.name.cell(),
                ingredient.is_active.cell(),
                ingredient.is_alcoholic.cell(),
                ingredient.amount.cell(),
                ingredient.description.clone().cell(),
                IngredientType::try_from(ingredient.ingredient_type)
                    .unwrap_or(IngredientType::Unspecified)
                    .cell(),
                ingredient
                    .regulator
                    .map_or("Not Set".to_string(), |fr| {
                        fr.fr_id.map_or("Not Set".to_string(), |id| id.to_string())
                    })
                    .cell(),
                ingredient
                    .instruction
                    .map_or("Not Set".to_string(), |instr| instr.id.to_string())
                    .cell(),
            ]);
        }
        table
            .table()
            .title(vec![
                "ID".cell().bold(true),
                "Name".cell().bold(true),
                "Is Active".cell().bold(true),
                "Is Alcoholic".cell().bold(true),
                "Amount".cell().bold(true),
                "Description".cell().bold(true),
                "Ingredient_Type".cell().bold(true),
                "FR ID".cell().bold(true),
                "Instruction ID".cell().bold(true),
            ])
            .bold(true)
    }

    fn get_schema_columns() {
        println!("{}", IngredientSchema::IngredientId);
    }
    fn sanatize_input(&self) -> UdmResult<Vec<FetchData>> {
        if self.query_options.is_none() {
            return Err(UdmError::InvalidInput(
                "Error while parsing query".to_string(),
            ));
        }
        let collected_queries =
            FetchData::to_fetch_data_vec(self.query_options.clone().unwrap().as_str())?;
        Ok(collected_queries)
    }
}
#[derive(Args, Debug)]
pub struct RemoveIngredientArgs {
    #[arg(short, long, required = true)]
    ingredient_id: i32,
    #[arg(
        short,
        long,
        help = "Does not prompt, you are absolutely sure",
        default_value = "true"
    )]
    yes: bool,
}
#[async_trait]
impl MainCommandHandler for RemoveIngredientArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if !self.yes {
            let _ = ensure_removal();
        }
        let req = RemoveIngredientRequest {
            ingredient_id: self.ingredient_id,
        };
        let mut open_conn = options.connect().await?;
        let response = open_conn.remove_ingredient(req).await;
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
