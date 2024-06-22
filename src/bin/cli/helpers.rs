use cli_table::TableStruct;
use lib::rpc_types::server::udm_service_client::UdmServiceClient;
use lib::rpc_types::service_types::FetchData;
use lib::UdmResult;

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
#[derive(Debug)]
pub struct UdmServerOptions {
    pub host: String,
    pub port: i64,
}

impl UdmServerOptions {
    pub async fn connect(self) -> UdmResult<UdmServiceClient<tonic::transport::channel::Channel>> {
        let udm_server = format!("http://{}:{}", self.host, self.port);
        let client = UdmServiceClient::connect(udm_server)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Could not connect to UDM Server {}", e);
                std::process::exit(1)
            });
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
