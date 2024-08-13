use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerService;
// use crate::rpc_types::drink_controller::drink_controller_service_server::DrinkControllerServiceServer;
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
use crate::rpc_types::server::GrpcServerFactory;
use crate::rpc_types::service_types::GenericEmpty;
use tonic::async_trait;
use tonic::Request;
use tonic::Response;
use tonic::Status;
// use tracing;
tonic::include_proto!("drink_ctrl_server");

pub struct DrinkControllerContext {}

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
}

impl GrpcServerFactory for DrinkControllerContext {}
