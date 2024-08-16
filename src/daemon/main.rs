use clap::Parser;
use config::Config;
use lib::logger::UdmLogger;
use lib::logger::UdmLoggerType;
use lib::parsers::settings::UdmConfigurer;
use lib::parsers::validate_configurer;
use lib::rpc_types::drink_controller::DrinkControllerServer;
use lib::rpc_types::server::GrpcServerFactory;
use lib::rpc_types::server::SqlDaemonServer;
use lib::FileRetrieve;
use lib::Retrieval;
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::runtime::Builder as RuntimeBuilder;
use tracing::debug;
use tracing::info;
pub mod cli;

async fn gen_sql_daemon_server(cloned_configeror: Arc<UdmConfigurer>) {
    let sql_daemon_addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        cloned_configeror.udm.port.try_into().unwrap(),
    );
    tracing::info!(
        "Attempting to start Sql Daemon Server on {}",
        &sql_daemon_addr
    );
    let sql_daemon_server = SqlDaemonServer::new(cloned_configeror, sql_daemon_addr);
    let _ = sql_daemon_server.start_server().await;
}
async fn gen_drink_controller_server(cloned_configeror: Arc<UdmConfigurer>) {
    let drink_controller_conf = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        cloned_configeror.drink_controller.port.try_into().unwrap(), // cloned_configeror.udm.port.try_into().unwrap(), Will uncomment when config is done
    );
    tracing::info!(
        "Attempting to start Drink Controller on {}",
        &drink_controller_conf
    );
    let drink_controler = DrinkControllerServer::new(cloned_configeror, drink_controller_conf);
    let _ = drink_controler.start_server().await;
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::DaemonCli::parse();

    let config_file = FileRetrieve::new(cli_opts.config_file.clone()).retreieve::<Config>()?;
    let configeror = Arc::new(config_file.try_deserialize::<UdmConfigurer>()?);
    UdmLogger::init(
        UdmLoggerType::Main,
        cli_opts.verbose,
        Some(configeror.daemon.log_file_path.as_str()),
        cli_opts.test,
    )?;
    info!(
        "Initialized logger, collected Config File {}",
        &cli_opts.config_file.display()
    );
    debug!("Using configuration: {:?}", &configeror);
    validate_configurer(Arc::clone(&configeror)).unwrap_or_else(|e| panic!("{}", e));
    // Building multiple processes with heart beats
    // The parent process will initize and check in with each server
    // Each child process with have multiple threads or multiple async threads
    // Load in the Correct Db Settings and establish connection
    let worker_threads = num_cpus::get();
    tracing::info!("Building run time with {} worker threads", worker_threads);
    // Build runtime
    let runtime = RuntimeBuilder::new_multi_thread()
        .enable_all()
        .worker_threads(worker_threads)
        .thread_name("Async Thread Pool")
        .build()?;
    tracing::debug!("Finished runtime {:?}", &runtime);
    let cloned_configeror = Arc::clone(&configeror);
    // build signals
    // let mut signals = runtime.block_on(async move { Signals::new([SIGINT, SIGTERM, SIGABRT]) })?;
    // let signal_handler = signals.handle();
    // Spawn Sql Daemon Server
    runtime.block_on(async move {
        let daemon_sql_task = tokio::spawn(gen_sql_daemon_server(cloned_configeror));
        let drink_contrl_task = tokio::spawn(gen_drink_controller_server(Arc::clone(&configeror)));
        tokio::select! {
            _ = daemon_sql_task => {
                tracing::info!("Finished sql daemon server");
            }
            _ = drink_contrl_task => {
                tracing::info!("Finished Drink Controller server");
            }
        }
        tracing::info!("Finished all tasks");
    });
    Ok(())
}
