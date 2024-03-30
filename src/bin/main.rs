extern crate log;
use clap::Parser;
use cli::helpers::UdmServerOptions;
use lib::logger;

use std::error::Error;

use crate::cli::helpers::MainCommandHandler;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::UdmCli::parse();
    logger::MyLogger::init(cli_opts.verbose, None)?;
    log::info!("Initialized logger");
    let server_options = UdmServerOptions {
        host: cli_opts.udm_server.to_string(),
        port: cli_opts.udm_port,
    };
    if let Some(commands) = &cli_opts.command {
        match commands {
            cli::UdmCommand::Recipe(_user_input) => todo!(),
            cli::UdmCommand::Ingredient(_user_input) => todo!(),
            cli::UdmCommand::Instruction(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Fluid(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Reset(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
        }
    }
    Ok(())
}
