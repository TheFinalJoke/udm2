use crate::cli::helpers::ensure_removal;
use async_trait::async_trait;
use clap::Args;
use clap::Subcommand;
use cli_table::Cell;
use cli_table::Style;
use cli_table::Table;
use cli_table::TableStruct;
use lib::db::InstructionSchema;
use lib::error::UdmError;
use lib::rpc_types::recipe_types::Instruction;
use lib::rpc_types::service_types::AddInstructionRequest;
use lib::rpc_types::service_types::CollectInstructionRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::ModifyInstructionRequest;
use lib::rpc_types::service_types::RemoveInstructionRequest;
use lib::rpc_types::FieldValidation;
use lib::UdmResult;

use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::ShowHandler;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::UdmServerOptions;

#[derive(Subcommand, Debug)]
pub enum InstructionCommands {
    #[command(about = "Add a Instruction")]
    Add(AddInstructionArgs),
    #[command(about = "Show Instructionn")]
    Show(ShowInstructionArgs),
    #[command(about = "Remove Instruction")]
    Remove(RemoveInstructionArgs),
    #[command(about = "Update instruction")]
    Update(UpdateInstructionArgs),
}
#[async_trait]
impl MainCommandHandler for InstructionCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            InstructionCommands::Add(user_input) => user_input.handle_command(options).await,
            InstructionCommands::Show(user_input) => user_input.handle_command(options).await,
            InstructionCommands::Remove(user_input) => user_input.handle_command(options).await,
            InstructionCommands::Update(user_input) => user_input.handle_command(options).await,
        }
    }
}
#[derive(Args, Debug)]
pub struct AddInstructionArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short = 'i', long)]
    instruction_id: Option<i32>,
    #[arg(short = 'n', long)]
    instruction_name: Option<String>,
    #[arg(short = 'd', long)]
    instruction_detail: Option<String>,
}

impl UdmGrpcActions<Instruction> for AddInstructionArgs {
    fn sanatize_input(&self) -> UdmResult<Instruction> {
        if let Some(raw_input) = &self.raw {
            tracing::debug!("Json passed: {}", &raw_input);
            let instruction: Instruction = serde_json::from_str(raw_input)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            instruction.validate_without_id_fields()?;
            return Ok(instruction);
        }

        if self.instruction_name.is_none() || self.instruction_detail.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(Instruction {
            id: self.instruction_id,
            instruction_detail: self.instruction_detail.clone(),
            instruction_name: self.instruction_name.clone(),
        })
    }
}
#[async_trait]
impl MainCommandHandler for AddInstructionArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let instruction = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_instruction(AddInstructionRequest {
                instruction: Some(instruction),
            })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Inserted into database, got ID back {}",
            response.into_inner().instruction_id
        );
        Ok(())
    }
}
#[derive(Args, Debug)]
pub struct ShowInstructionArgs {
    query_options: Option<String>,
    #[arg(long, short = 'e', help = "Example queries", default_value = "false")]
    example: bool,
    #[arg(long, short = 's', help = "show_fields", default_value = "false")]
    show_fields: bool,
}
#[async_trait]
impl MainCommandHandler for ShowInstructionArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if self.example {
            ShowInstructionArgs::show_example();
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
            let mut open_connection = options.connect().await?;
            let response = open_connection
                .collect_instructions(CollectInstructionRequest {
                    expressions: fetched,
                })
                .await
                .map_err(|e| UdmError::ApiFailure(format!("{}", e)));
            match response {
                Ok(response) => {
                    tracing::debug!("Got response {:?}", &response);
                    let instructions = response.into_inner().instructions;
                    println!("Found {} results", &instructions.len());
                    let table = self.create_tables(instructions);
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
impl ShowHandler<Instruction> for ShowInstructionArgs {
    fn show_example() {
        println!("To build a query it will be <field><operation><values>");
        println!("fr_id=1");
        println!("^^ will query fr_id=1");
        println!("Another example of multiple values, fr_id = 1");
        Self::get_schema_columns();
    }

    fn create_tables(&self, data: Vec<Instruction>) -> TableStruct {
        let mut table = Vec::new();
        for instruction in data {
            let instruction_id = match &instruction.id {
                Some(id) => format!("{}", id),
                None => "Not Set".to_string(),
            };
            let name = match &instruction.instruction_name {
                Some(name) => name.to_string(),
                None => "Not Set".to_string(),
            };
            let detail = match instruction.instruction_detail {
                Some(name) => name.to_string(),
                None => "Not Set".to_string(),
            };
            table.push(vec![instruction_id.cell(), name.cell(), detail.cell()]);
        }
        table
            .table()
            .title(vec![
                "ID".cell().bold(true),
                "Name".cell().bold(true),
                "Details".cell().bold(true),
            ])
            .bold(true)
    }

    fn get_schema_columns() {
        println!("{}", InstructionSchema::InstructionId);
    }
}
impl ShowInstructionArgs {
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
pub struct RemoveInstructionArgs {
    #[arg(short, long)]
    instruction_id: Option<i32>,
}
#[async_trait]
impl MainCommandHandler for RemoveInstructionArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let id = self.instruction_id.ok_or_else(|| {
            UdmError::InvalidInput("Invalid input to remove fluid regulator".to_string())
        })?;
        let _ = ensure_removal();
        let req = RemoveInstructionRequest { instruction_id: id };
        let mut open_conn = options.connect().await?;
        let response = open_conn.remove_instruction(req).await;
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
pub struct UpdateInstructionArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short, long, help = "Specify the ID")]
    instruction_id: Option<i32>,
    #[arg(short = 'd', long = "details", help = "Instruction Details")]
    detail: Option<String>,
    #[arg(short = 'n', long = "name", help = "Instruction Name")]
    name: Option<String>,
}
impl UdmGrpcActions<Instruction> for UpdateInstructionArgs {
    fn sanatize_input(&self) -> UdmResult<Instruction> {
        if let Some(raw_input) = &self.raw {
            tracing::debug!("Json passed: {}", &raw_input);
            let instruction: Instruction = serde_json::from_str(raw_input)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            instruction.validate_all_fields()?;
            return Ok(instruction);
        }
        if self.instruction_id.is_none() || self.detail.is_none() || self.name.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(Instruction {
            id: self.instruction_id,
            instruction_detail: self.detail.clone(),
            instruction_name: self.name.clone(),
        })
    }
}
#[async_trait]
impl MainCommandHandler for UpdateInstructionArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let instruction = self.sanatize_input().unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .update_instruction(ModifyInstructionRequest {
                instruction: Some(instruction),
            })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        tracing::debug!("Got response {:?}", response);
        tracing::info!(
            "Updated database, got ID back {}",
            response.into_inner().instruction_id
        );
        Ok(())
    }
}
