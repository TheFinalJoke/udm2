use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::UdmServerOptions;
use clap::{Args, Subcommand};
use lib::error::UdmError;
use lib::rpc_types::fhs_types;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::rpc_types::service_types;
use lib::UdmResult;
use tonic::async_trait;

#[derive(Subcommand, Debug)]
pub enum FluidCommands {
    #[command(about = "Add a fluid regulator")]
    Add(AddFluidArgs),
    #[command(about = "Show current fluid regulators")]
    Show(ShowFluidArgs),
    #[command(about = "Remove a fluid regulator")]
    Remove(RemoveFluidArgs),
}
#[async_trait]
impl MainCommandHandler for FluidCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            FluidCommands::Add(user_input) => user_input.handle_command(options).await,
            FluidCommands::Show(_user_input) => todo!(),
            FluidCommands::Remove(_user_input) => todo!(),
        }
    }
}

#[derive(Args, Debug)]
pub struct AddFluidArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short, long, default_value = "false")]
    json: bool,
    #[arg(short, long, help = "Specify the ID")]
    fr_id: Option<i64>,
    #[arg(short='t', long="type", help="Type of regulator", value_parser=fhs_types::RegulatorType::get_possible_values())]
    reg_type: Option<String>,
    #[arg(
        short = 'g',
        long = "gpio_pin",
        help = "The GPIO pin the device is connected"
    )]
    gpio_pin: Option<i32>,
}
impl UdmGrpcActions<FluidRegulator> for AddFluidArgs {
    fn sanatize_input(&self) -> lib::UdmResult<FluidRegulator> {
        if let Some(raw_input) = &self.raw {
            log::debug!("Json passed: {}", &raw_input);
            let fluid = serde_json::from_str(raw_input)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")));
            return fluid;
        } else if self.fr_id.is_none() || self.reg_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(FluidRegulator {
            fr_id: Some(self.fr_id.unwrap()),
            regulator_type: Some(fhs_types::RegulatorType::from_str_name(
                self.reg_type.clone().unwrap().as_str(),
            ).unwrap().into()),
            gpio_pin: self.gpio_pin,
        })
    }
}
#[async_trait]
impl MainCommandHandler for AddFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fr = self.sanatize_input().unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_fluid_regulator(service_types::AddFluidRegulatorRequest { fluid: Some(fr) })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        log::info!("Inserted into database");
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ShowFluidArgs {
    #[arg(short, long, default_value = "false")]
    json: bool,
    #[arg(short, long, help = "Look up regulators by ID")]
    fr_id: Option<i64>,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Shows all available recipes"
    )]
    show_all: bool,
}

#[derive(Args, Debug)]
pub struct RemoveFluidArgs {
    #[arg(short, long, help = "Remove fluid regulator by ID", required = true)]
    fr_id: Option<i64>,
}
