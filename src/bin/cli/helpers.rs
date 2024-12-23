use cli_table::TableStruct;
use lib::rpc_types::drink_controller::drink_controller_service_client::DrinkControllerServiceClient;
use lib::rpc_types::server::udm_service_client::UdmServiceClient;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::{DrinkServerBuilder, SqlUdmServerBuilder};
use lib::UdmResult;
use tonic::transport::channel::Channel;

pub trait UdmGrpcActions<T> {
    fn sanatize_input(&self) -> UdmResult<T>;
}

#[async_trait::async_trait]
pub trait MainCommandHandler {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()>;
}

pub(crate) trait ShowHandler<T>: MainCommandHandler {
    fn show_example();
    fn create_tables(&self, data: Vec<T>) -> TableStruct;
    fn get_schema_columns();
    fn sanatize_input(&self) -> UdmResult<Vec<FetchData>>;
}
pub trait ServerOptions {
    fn new(host: String, port: i64) -> Self;
}
#[derive(Debug)]
pub struct SqlUdmServerCliOptions {
    pub udm_host: String,
    pub udm_port: i64,
}
impl ServerOptions for SqlUdmServerCliOptions {
    fn new(host: String, port: i64) -> Self {
        Self {
            udm_host: host,
            udm_port: port,
        }
    }
}
#[derive(Debug)]
pub struct DrinkControllerServerCliOptions {
    pub host: String,
    pub port: i64,
}
impl ServerOptions for DrinkControllerServerCliOptions {
    fn new(host: String, port: i64) -> Self {
        Self { host, port }
    }
}
#[derive(Debug)]
pub struct UdmServerOptions {
    pub(crate) sql_udm_server: SqlUdmServerCliOptions,
    pub(crate) drink_server: DrinkControllerServerCliOptions,
}

impl UdmServerOptions {
    pub async fn connect_to_udm(self) -> UdmResult<UdmServiceClient<Channel>> {
        let client =
            SqlUdmServerBuilder::new(self.sql_udm_server.udm_host, self.sql_udm_server.udm_port)
                .connect()
                .await?;
        Ok(client)
    }
    pub async fn connect_to_drink_server(self) -> UdmResult<DrinkControllerServiceClient<Channel>> {
        let client = DrinkServerBuilder::new(self.drink_server.host, self.drink_server.port)
            .connect()
            .await?;
        Ok(client)
    }
}
pub fn ensure_removal() -> UdmResult<()> {
    let mut buffer = String::new();
    println!("Are you sure you want to remove? y/n");
    let _ = std::io::stdin().read_line(&mut buffer);
    let input = buffer.trim().to_owned();
    if &input == "y" {
        Ok(())
    } else {
        std::process::exit(2)
    }
}
