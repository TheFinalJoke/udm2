extern crate log;
use clap::Parser;
use cli::helpers::DrinkControllerServerCliOptions;
use cli::helpers::ServerOptions;
use cli::helpers::SqlUdmServerCliOptions;
use cli::helpers::UdmServerOptions;
use lib::logger::UdmLogger;
use lib::logger::UdmLoggerType;

use std::error::Error;

use crate::cli::helpers::MainCommandHandler;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::UdmCli::parse();
    UdmLogger::init(UdmLoggerType::Bin, cli_opts.verbose, None, false)?;
    tracing::info!("Initialized logger");
    let server_options = UdmServerOptions {
        sql_udm_server: SqlUdmServerCliOptions::new(
            cli_opts.udm_server.to_string(),
            cli_opts.udm_port,
        ),
        drink_server: DrinkControllerServerCliOptions::new(
            cli_opts.drink_server.to_string(),
            cli_opts.drink_ctrl_port,
        ),
    };
    tracing::info!("Server cli options created: {:?}", &server_options);
    if let Some(commands) = &cli_opts.command {
        match commands {
            cli::UdmCommand::Recipe(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Ingredient(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Instruction(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Fluid(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::RecipeToInstruction(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
            cli::UdmCommand::Reset(user_input) => {
                let _ = user_input.handle_command(server_options).await;
            }
        }
    }
    Ok(())
}
