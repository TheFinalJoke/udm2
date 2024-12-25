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
use tokio::sync::Notify;
use tracing::debug;
use tracing::info;
pub mod cli;

#[derive(Clone)]
struct GenerateDrinkControllerServer {
    configerator: Arc<UdmConfigurer>,
    notify: Arc<Notify>,
}
impl GenerateDrinkControllerServer {
    fn new(configerator: Arc<UdmConfigurer>, notify: Arc<Notify>) -> Self {
        Self {
            configerator,
            notify,
        }
    }
    async fn gen_drink_controller_server(&self) {
        let drink_controller_conf = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            self.configerator.drink_controller.port.try_into().unwrap(),
        );
        tracing::info!(
            "Attempting to start Drink Controller on {}",
            &drink_controller_conf
        );
        self.notify.notified().await;
        tracing::info!("Recieved Notification to start Drink Server, because SQL Server is up");
        let drink_controller =
            DrinkControllerServer::new(self.configerator.clone(), drink_controller_conf);
        let _ = drink_controller.start_server().await;
    }
}
#[derive(Clone)]
struct GenerateSqlDaemonServer {
    configerator: Arc<UdmConfigurer>,
    notify: Arc<Notify>,
}

impl GenerateSqlDaemonServer {
    fn new(configerator: Arc<UdmConfigurer>, notify: Arc<Notify>) -> Self {
        Self {
            configerator,
            notify,
        }
    }
    async fn gen_sql_daemon_server(&self) {
        let sql_daemon_addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            self.configerator.udm.port.try_into().unwrap(),
        );
        tracing::info!(
            "Attempting to start Sql Daemon Server on {}",
            &sql_daemon_addr
        );
        let sql_daemon_server = SqlDaemonServer::new(
            self.configerator.clone(),
            sql_daemon_addr,
            Some(self.notify.clone()),
        );
        let _ = sql_daemon_server.start_server().await;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::DaemonCli::parse();

    let config_file = FileRetrieve::new(cli_opts.config_file.clone()).retreieve::<Config>()?;
    let configerator = Arc::new(config_file.try_deserialize::<UdmConfigurer>()?);
    UdmLogger::init(
        UdmLoggerType::Daemon,
        cli_opts.verbose,
        Some(configerator.daemon.log_file_path.as_str()),
    )?;
    info!(
        "Initialized logger, collected Config File {}",
        &cli_opts.config_file.display()
    );
    debug!("Using configuration: {:?}", &configerator);
    validate_configurer(Arc::clone(&configerator)).unwrap_or_else(|e| panic!("{}", e));
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
    let notify = Arc::new(Notify::new());
    // Spawn Sql Daemon Server
    runtime.block_on(async move {
        let sql_server = Arc::new(GenerateSqlDaemonServer::new(
            Arc::clone(&configerator),
            Arc::clone(&notify),
        ));
        let drink_server =
            GenerateDrinkControllerServer::new(Arc::clone(&configerator), notify.clone());
        let daemon_sql_task = tokio::spawn(async move { sql_server.gen_sql_daemon_server().await });
        let drink_contrl_task =
            tokio::spawn(async move { drink_server.gen_drink_controller_server().await });
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
