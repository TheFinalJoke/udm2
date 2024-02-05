extern crate log;
use clap::Parser;
use lib::logger;
use cli::helpers::UdmServerOptions;
use lib::rpc_types::service_types::AddFluidRegulatorRequest;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::rpc_types::fhs_types::RegulatorType;
use std::error::Error;

use crate::cli::helpers::MainCommandHandler;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::UdmCli::parse();
    let debug_level = logger::get_log_level(cli_opts.debug);
    logger::MyLogger::init(debug_level).unwrap();
    log::info!(
        "Initialized logger, collecting Config File {}",
        &cli_opts.config_file.display()
    );
    let server_options = UdmServerOptions{
            host: cli_opts.udm_server.to_string(),
            port: cli_opts.udm_port
        };
    if let Some(commands) = &cli_opts.command {
        match commands {
            cli::UdmCommand::Recipe(user_input) => todo!(),
            cli::UdmCommand::Ingredient(user_input) => todo!(),
            cli::UdmCommand::Instruction(user_input) => todo!(),
            cli::UdmCommand::Fluid(user_input) => {
                let _ = user_input.handle_command(server_options);
            },
        }
    }
    // let request = AddFluidRegulatorRequest{
    //     fluid: Some(FluidRegulator{
    //         fr_id: 1,
    //         gpio_pin: 10,
    //         regulator_type: RegulatorType::Valve.into()
    //     })
    // };
    // let response = client.add_fluid_regulator(request).await?;
    // // println!("metadata {:?}", &response.metadata());
    // // let (metadata, mess, ext) = response.into_parts();
    // println!("response: {:?}, client {:?}", response.into_inner(), client);
    Ok(())
}
