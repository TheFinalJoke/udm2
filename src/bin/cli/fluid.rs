use clap::{Args, Subcommand};
use lib::rpc_types::fhs_types;
use lib::rpc_types::fhs_types::FluidRegulator;
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
impl MainCommandHandler<UdmResult<()>> for FluidCommands {
    fn handle_command(&self) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            FluidCommands::Add(user_input) => todo!(),
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
    gpio_pin: Option<i32>,
}
impl UdmGrpcActions for AddFluidArgs {
    fn sanatize_input(&self) -> lib::UdmResult<()> {
        if let Some(raw_input) = &self.raw {
            let fluid: fhs_types::FluidRegulator = serde_json::from_str(raw_input).unwrap();
            println!("{:?}", fluid);
        }
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