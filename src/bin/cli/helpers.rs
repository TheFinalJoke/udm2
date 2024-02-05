use log;
use lib::UdmResult;
use lib::rpc_types::server::udm_service_client::UdmServiceClient;

pub trait UdmGrpcActions<T> {
    fn sanatize_input(&self) -> UdmResult<T>;
}

pub trait MainCommandHandler {
    fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()>;
}

#[derive(Debug)]
pub struct UdmServerOptions {
    pub host: String,
    pub port: i64,
}

impl UdmServerOptions {
    pub async fn connect(self) -> UdmResult<UdmServiceClient<tonic::transport::channel::Channel>> {
        let udm_server = format!("http://{}:{}", self.host, self.port);
        let client = UdmServiceClient::connect(udm_server).await.unwrap_or_else(|e| {
            log::error!("Could not connect to UDM Server {}", e);
            std::process::exit(1)
        });
        Ok(client)
    }
}