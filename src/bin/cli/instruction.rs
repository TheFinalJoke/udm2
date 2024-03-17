use async_trait::async_trait;
use clap::Args;
use clap::Subcommand;
use lib::error::UdmError;
use lib::rpc_types::recipe_types::Instruction;
use lib::rpc_types::service_types::AddInstructionRequest;
use lib::rpc_types::service_types::ModifyInstructionRequest;
use lib::rpc_types::FieldValidation;
use lib::UdmResult;

use crate::cli::helpers::MainCommandHandler;
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
            InstructionCommands::Show(_user_input) => todo!(),
            InstructionCommands::Remove(_user_input) => todo!(),
            InstructionCommands::Update(_user_input) => todo!(),
        }
    }
}
#[derive(Args, Debug)]
pub struct AddInstructionArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short='i', long)]
    instruction_id: Option<i32>,
    #[arg(short='n', long)]
    instruction_name: Option<String>,
    #[arg(short='d', long)]
    instruction_detail: Option<String>,
}

impl UdmGrpcActions<Instruction> for AddInstructionArgs {
    fn sanatize_input(&self) -> UdmResult<Instruction> {
        if let Some(raw_input) = &self.raw {
            log::debug!("Json passed: {}", &raw_input);
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
            log::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_instruction(AddInstructionRequest {
                instruction: Some(instruction),
            })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        log::info!(
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
}

#[derive(Args, Debug)]
pub struct RemoveInstructionArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    instruction_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
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
            log::debug!("Json passed: {}", &raw_input);
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
            log::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .update_instruction(ModifyInstructionRequest { instruction: Some(instruction) })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        log::info!(
            "Updated database, got ID back {}",
            response.into_inner().instruction_id
        );
        Ok(())
    }
}