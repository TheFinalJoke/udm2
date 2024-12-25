use crate::cli::helpers::MainCommandHandler;
use async_trait::async_trait;
use clap::Args;
use clap::Subcommand;
use lib::error::trace_log_error;
use lib::error::UdmError;
use lib::rpc_types::drink_ctrl_types::GetPumpGpioInfoRequest;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::UdmResult;

use super::helpers::UdmServerOptions;

#[derive(Subcommand, Debug)]
pub enum DrinkServer {
    #[command(about = "Collect Pump or tap information")]
    CollectPump(CollectPumpInfoArgs),
}

#[async_trait]
impl MainCommandHandler for DrinkServer {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            DrinkServer::CollectPump(user_input) => user_input.handle_command(options).await,
        }
    }
}
// Args
#[derive(Args, Debug)]
pub struct CollectPumpInfoArgs {
    #[arg(short = 'p', long, help = "Pump Number to Collect", exclusive = true)]
    pub pump_number: Option<i32>,
    #[arg(short = 'g', long, help = "Gpio Pin to Collect", exclusive = true)]
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
        tracing::info!("Collected Request {:?}", req);
        let mut open_connection = options.connect_to_drink_server().await?; // How do i open up a a drink controller
        tracing::debug!("Opened connection with Drink Server, Sending request to collect Pump info and current state");
        let response = open_connection.get_pump_gpio_info(req).await.map_err(|e| {
            let message = format!("Fatal Failure on server: {}", e);
            trace_log_error(UdmError::ApiFailure(message.clone()))
        })?;
        tracing::debug!("Raw Response: {:?} ", &response);
        println!("{}", response.into_inner());
        Ok(())
    }
}
