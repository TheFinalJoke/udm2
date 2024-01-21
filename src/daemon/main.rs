extern crate log;
use clap::Parser;

use lib::db;
use lib::rpc_types::server;
use lib::rpc_types::service_types;
use lib::{logger, parsers, Retrieval};
use std::error::Error;
use std::rc::Rc;
use tonic::{transport::Server, Request, Response, Status};

pub mod cli;

#[derive(Default)]
pub struct UdmService {}

#[tonic::async_trait]
impl server::udm_service_server::UdmService for UdmService {
    async fn add_fluid_regulator(
        &self,
        _request: Request<service_types::AddFluidRegulatorRequest>,
    ) -> Result<Response<service_types::AddFluidRegulatorResponse>, Status> {
        let reply = service_types::AddFluidRegulatorResponse { fu_id: 132 };
        Ok(Response::new(reply))
    }
}
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
    let connection = db_type.establish_connection();
    log::info!("Initializing database");
    let _ = connection
        .await
        .gen_schmea()
        .await
        .map_err(|e| format!("Failed to create database schema {}", e));

    let addr = format!("127.0.0.1:{}", Rc::clone(&configeror).udm.port).parse()?;
    let udm_service = UdmService::default();
    log::info!("Running Udm Service on {:?}", addr);
    Server::builder()
        .add_service(server::udm_service_server::UdmServiceServer::new(
            udm_service,
        ))
        .serve(addr)
        .await?;
    Ok(())
}
