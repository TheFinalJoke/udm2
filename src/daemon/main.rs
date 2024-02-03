extern crate log;
use clap::Parser;
use lib::db;
use lib::rpc_types::server;
use lib::{logger, parsers, Retrieval};
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::rc::Rc;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::DaemonCli::parse();
    let debug_level = logger::get_log_level(cli_opts.debug);
    logger::MyLogger::init(debug_level)?;
    log::info!(
        "Initialized logger, collecting Config File {}",
        &cli_opts.config_file.display()
    );
    let config_file = lib::FileRetrieve::new(cli_opts.config_file).retreieve::<config::Config>()?;

    let configeror = Rc::new(config_file.try_deserialize::<parsers::settings::UdmConfigurer>()?);
    log::debug!("Using configuration: {:?}", &configeror);
    lib::parsers::validate_configurer(Rc::clone(&configeror)).unwrap_or_else(|e| panic!("{}", e));
    // Load in the Correct Db Settings and establish connection
    let db_type = db::DbType::load_db(Rc::clone(&configeror));
    let mut connection = db_type.establish_connection().await;
    log::info!("Initializing database");
    let _ = connection
        .gen_schmea()
        .await
        .map_err(|e| format!("Failed to create database schema {}", e));

    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Rc::clone(&configeror).udm.port.try_into()?,
    );
    log::info!("Attempting to start server on {}", &addr);
    let daemon_server = server::DaemonServer::new(connection, addr);
    let udm_service = server::udm_service_server::UdmServiceServer::new(daemon_server);
    server::start_server(udm_service, addr).await?;
    Ok(())
}
