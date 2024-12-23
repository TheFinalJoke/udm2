use crate::cli::helpers::MainCommandHandler;
use clap::Args;
use clap::Subcommand;
use lib::error::UdmError;
use lib::rpc_types::drink_ctrl_types::GetPumpGpioInfoRequest;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::UdmResult;
use tonic::async_trait;

use super::helpers::UdmServerOptions;

#[derive(Subcommand, Debug)]
pub enum DrinkServer {
    #[command(about = "Collect Pump or tap information")]
    CollectPump(CollectPumpInfoArgs),
}

// Args
#[derive(Args, Debug)]
pub struct CollectPumpInfoArgs {
    pub pump_number: Option<i32>,
    pub gpio_pin: Option<i32>,
}

#[async_trait]
impl MainCommandHandler for CollectPumpInfoArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fr = FluidRegulator {
            pump_num: self.pump_number,
            gpio_pin: self.gpio_pin,
            ..Default::default()
        };
        let req = GetPumpGpioInfoRequest { fr: Some(fr) };
        let mut open_connection = options.connect_to_drink_server().await?; // How do i open up a a drink controller
        let response = open_connection
            .get_pump_gpio_info(req)
            .await
            .map_err(|e| UdmError::ApiFailure(format!("Fatal Failure on server: {}", e)))?;
        tracing::debug!("Raw Response: {:?} ", &response);
        println!("{}", response.into_inner());
        Ok(())
    }
}
