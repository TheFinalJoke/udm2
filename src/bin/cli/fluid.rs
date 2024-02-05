use clap::{Args, Subcommand};
use lib::error::UdmError;
use lib::rpc_types::fhs_types;
use lib::rpc_types::fhs_types::FluidRegulator;
use crate::cli::helpers::UdmServerOptions;
use lib::UdmResult;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::MainCommandHandler;

#[derive(Subcommand, Debug)]
pub enum FluidCommands {
    #[command(about = "Add a fluid regulator")]
    Add(AddFluidArgs),
    #[command(about = "Show current fluid regulators")]
    Show(ShowFluidArgs),
    #[command(about = "Remove a fluid regulator")]
    Remove(RemoveFluidArgs)
}
impl MainCommandHandler for FluidCommands {
    fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            FluidCommands::Add(user_input) => {
                user_input.handle_command(options)
            },
            FluidCommands::Show(user_input) => todo!(),
            FluidCommands::Remove(user_input) => todo!(),
        }
    }
}


#[derive(Args, Debug)]
pub struct AddFluidArgs {
    #[arg(long, value_name="JSON", help="Raw json to transform")]
    raw: Option<String>,
    #[arg(short, long, default_value="false")]
    json: bool,
    #[arg(short, long, help="Specify the ID")]
    fr_id: Option<i64>,
    #[arg(short='t', long="type", help="Type of regulator", value_parser=fhs_types::RegulatorType::get_possible_values())]
    reg_type: Option<i32>,
    #[arg(short='g', long="gpio_pin", help="The GPIO pin the device is connected")]
    gpio_pin: Option<i32>
}
impl UdmGrpcActions<FluidRegulator> for AddFluidArgs {
    fn sanatize_input(&self) -> lib::UdmResult<FluidRegulator> {
        if let Some(raw_input) = &self.raw {
            let fluid  = serde_json::from_str(raw_input).map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")));
            return fluid
        }
        else if self.fr_id.is_none() || self.reg_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from("Invalid input")))
        }
        Ok(
            FluidRegulator{
                fr_id: self.fr_id.unwrap(),
                regulator_type: self.reg_type.unwrap().into(),
                gpio_pin: self.gpio_pin.unwrap()
            }
        )
    }
}
impl MainCommandHandler for AddFluidArgs {
    fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fr = self.sanatize_input().unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(2)
        });
        println!("{:?}", fr);
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ShowFluidArgs {
    #[arg(short, long, default_value="false")]
    json: bool,
    #[arg(short, long, help="Look up regulators by ID")]
    fr_id: Option<i64>,
    #[arg(short, long, default_value="false", help="Shows all available recipes")]
    show_all: bool
}

#[derive(Args, Debug)]
pub struct RemoveFluidArgs {
    #[arg(short, long, help="Remove fluid regulator by ID", required=true)]
    fr_id: Option<i64>,
}