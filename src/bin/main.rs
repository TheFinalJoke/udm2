extern crate log;
use clap::Parser;
use lib::logger;
use lib::rpc_types::server::udm_service_client::UdmServiceClient;
use std::error::Error;

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

    let udm_server = format!("{}:{}", cli_opts.udm_server.to_string(), cli_opts.udm_port);
    let client = UdmServiceClient::connect(udm_server).await?;

    Ok(())
}
