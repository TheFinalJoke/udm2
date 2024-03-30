extern crate log;
use clap::Parser;
use lib::db;
use lib::db::DbMetaData;
use lib::rpc_types::server;
use lib::{logger, parsers, Retrieval};
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::DaemonCli::parse();

    let config_file =
        lib::FileRetrieve::new(cli_opts.config_file.clone()).retreieve::<config::Config>()?;
    let configeror = Arc::new(config_file.try_deserialize::<parsers::settings::UdmConfigurer>()?);
    logger::MyLogger::init(
        cli_opts.verbose,
        Some(configeror.daemon.log_file_path.as_str()),
    )?;
    log::info!(
        "Initialized logger, collected Config File {}",
        &cli_opts.config_file.display()
    );
    log::debug!("Using configuration: {:?}", &configeror);
    lib::parsers::validate_configurer(Arc::clone(&configeror)).unwrap_or_else(|e| panic!("{}", e));
    // Load in the Correct Db Settings and establish connection
    let db_type = Arc::new(db::DbType::load_db(Arc::clone(&configeror)));
    let mut connection = db_type.establish_connection().await;
    log::info!("Initializing database");
    let _ = connection
        .gen_schmea()
        .await
        .map_err(|e| format!("Failed to create database schema {}", e));

    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Arc::clone(&configeror).udm.port.try_into()?,
    );
    log::info!("Attempting to start server on {}", &addr);
    let db_metadata = DbMetaData::new(Arc::clone(&db_type));
    let daemon_server = server::DaemonServerContext::new(connection, addr, db_metadata);
    let udm_service = server::udm_service_server::UdmServiceServer::new(daemon_server);
    server::start_server(udm_service, addr).await?;
    Ok(())
}
