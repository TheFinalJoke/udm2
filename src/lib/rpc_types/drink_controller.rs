use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerService;
use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerServiceServer;
use crate::rpc_types::drink_ctrl_types::CleanCycleRequest;
use crate::rpc_types::drink_ctrl_types::CleanCycleResponse;
// use crate::rpc_types::drink_ctrl_types::CleanType;
use crate::rpc_types::drink_ctrl_types::DispenseDrinkRequest;
use crate::rpc_types::drink_ctrl_types::DispenseDrinkResponse;
use crate::rpc_types::drink_ctrl_types::GetPumpGpioInfoRequest;
use crate::rpc_types::drink_ctrl_types::GetPumpGpioInfoResponse;
use crate::rpc_types::fhs_types::FluidRegulator;
// use crate::rpc_types::gpio_types::gpio_value::Value;
// use crate::rpc_types::gpio_types::GpioDirection;
// use crate::rpc_types::gpio_types::GpioMetadata;
// use crate::rpc_types::gpio_types::GpioState;
// use crate::rpc_types::gpio_types::GpioValue;
use crate::db::DbConnection;
use crate::db::DbMetaData;
// use crate::db::DbType;
use crate::parsers::settings::UdmConfigurer;
use crate::rpc_types::drink_ctrl_types::PollDrinkStreamRequest;
use crate::rpc_types::drink_ctrl_types::PollDrinkStreamResponse;
use crate::rpc_types::server::GrpcServerFactory;
use crate::rpc_types::service_types::GenericEmpty;
use crate::UdmResult;
use futures::stream::StreamExt;
use signal_hook_tokio::SignalsInfo;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::async_trait;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;
// use tracing;
tonic::include_proto!("drink_ctrl_server");

pub struct DrinkControllerContext {
    // pub connection: Box<dyn DbConnection>,
    pub addr: SocketAddr,
    // pub metadata: DbMetaData,
}
impl DrinkControllerContext {
    // pub fn new(connection: Box<dyn DbConnection>, addr: SocketAddr, metadata: DbMetaData) -> Self {
    //     Self {
    //         // connection,
    //         addr,
    //         // metadata,
    //     }
    // }
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}
pub struct DrinkControllerServer {
    configuration: Arc<UdmConfigurer>,
    addr: SocketAddr,
}
#[async_trait::async_trait]
impl GrpcServerFactory<DrinkControllerContext> for DrinkControllerServer {
    fn new(config: Arc<UdmConfigurer>, addr: SocketAddr) -> Self {
        Self {
            configuration: config,
            addr,
        }
    }
    async fn build_context(&self) -> DrinkControllerContext {
        // let db_type = Arc::new(DbType::load_db(Arc::clone(&self.configuration)));
        // let mut connection = db_type.establish_connection().await;
        tracing::info!("Initializing database");
        // let _ = connection
        //     .gen_schmea()
        //     .await
        //     .map_err(|e| format!("Failed to create database schema {}", e));
        tracing::info!("Attempting to Drink Controller Service on {}", self.addr);
        // let db_metadata = DbMetaData::new(Arc::clone(&db_type));
        // DrinkControllerContext::new(connection, self.addr, db_metadata)
        DrinkControllerContext::new(self.addr)
    }
    async fn start_server(&self) -> UdmResult<()> {
        let drink_contrl_server = self.build_context().await;
        let dc_service = DrinkControllerServiceServer::new(drink_contrl_server);
        tracing::info!("Running Drink Controller on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(dc_service)
            .serve(self.addr)
            .await;
        Ok(())
    }
    async fn start_server_with_signal(&self, mut signal: SignalsInfo) -> UdmResult<()> {
        let drink_contrl_server = self.build_context().await;
        let dc_service = DrinkControllerServiceServer::new(drink_contrl_server);
        tracing::info!("Running Drink Controller Service on {:?}", self.addr);
        let _ = Server::builder()
            .add_service(dc_service)
            .serve_with_shutdown(self.addr, async {
                let _ = signal.next().await;
                tracing::info!("Got a termination signal");
            })
            .await;
        Ok(())
    }
}

#[async_trait]
impl DrinkControllerService for DrinkControllerContext {
    async fn dispense_drink(
        &self,
        _request: Request<DispenseDrinkRequest>,
    ) -> Result<Response<DispenseDrinkResponse>, Status> {
        todo!()
    }
    async fn clean_cycle(
        &self,
        _request: Request<CleanCycleRequest>,
    ) -> Result<Response<CleanCycleResponse>, Status> {
        todo!()
    }
    async fn get_pump_gpio_info(
        &self,
        _request: Request<GetPumpGpioInfoRequest>,
    ) -> Result<Response<GetPumpGpioInfoResponse>, Status> {
        todo!()
    }
    async fn stop_emergency(
        &self,
        _request: Request<FluidRegulator>,
    ) -> Result<Response<GenericEmpty>, Status> {
        todo!()
    }
    async fn poll_drink_stream(
        &self,
        _request: Request<PollDrinkStreamRequest>,
    ) -> Result<Response<PollDrinkStreamResponse>, Status> {
        todo!()
    }
}
