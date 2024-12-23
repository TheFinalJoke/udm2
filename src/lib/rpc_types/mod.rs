// Note: The Module must be the same name as the package in proto

use std::fmt::Display;

use crate::UdmResult;
use std::fmt::Debug;
pub mod drink_controller;
pub mod drink_ctrl_types;
pub mod fhs_types;
pub mod gpio_types;
pub mod recipe_types;
pub mod server;
pub mod service_types;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::recipe_types::Ingredient;
use crate::rpc_types::recipe_types::Instruction;
use crate::rpc_types::recipe_types::Recipe;
use crate::rpc_types::server::udm_service_client::UdmServiceClient;
use drink_controller::drink_controller_service_client::DrinkControllerServiceClient;
use tonic::transport::channel::Channel;

pub trait FieldValidation {
    fn validate_all_fields(&self) -> UdmResult<()>;
    fn validate_without_id_fields(&self) -> UdmResult<()>;
}

pub trait MultipleValues: Display {
    fn get_possible_values() -> Vec<&'static str>;
}

/// This is for "main structures" for easy validation and creation
pub trait ProtoGen: Debug + FieldValidation {}

impl ProtoGen for FluidRegulator {}
impl ProtoGen for Instruction {}
impl ProtoGen for Ingredient {}
impl ProtoGen for Recipe {}

pub struct SqlUdmServerBuilder {
    pub(crate) host: String,
    pub(crate) port: i64,
}

impl SqlUdmServerBuilder {
    pub fn new(host: String, port: i64) -> SqlUdmServerBuilder {
        Self { host, port }
    }
    pub async fn connect(self) -> UdmResult<UdmServiceClient<Channel>> {
        let udm_server = format!("http://{}:{}", self.host, self.port);
        let client = UdmServiceClient::connect(udm_server)
            .await
            .unwrap_or_else(|e| {
                tracing::error!(
                    "Could not connect on Drink server on {}:{} with error: {:?}",
                    self.host,
                    self.port,
                    e
                );
                std::process::exit(1)
            });
        Ok(client)
    }
}
pub struct DrinkServerBuilder {
    pub(crate) host: String,
    pub(crate) port: i64,
}

impl DrinkServerBuilder {
    pub fn new(host: String, port: i64) -> DrinkServerBuilder {
        Self { host, port }
    }
    pub async fn connect(self) -> UdmResult<DrinkControllerServiceClient<Channel>> {
        let drink_server = format!("http://{}:{}", self.host, self.port);
        let client = DrinkControllerServiceClient::connect(drink_server)
            .await
            .unwrap_or_else(|e| {
                tracing::error!(
                    "Could not connect on Drink server on {}:{} with error: {:?}",
                    self.host,
                    self.port,
                    e
                );
                std::process::exit(1)
            });
        Ok(client)
    }
}
