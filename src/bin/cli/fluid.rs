use crate::cli::helpers::ensure_removal;
use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::UdmServerOptions;
use clap::Args;
use clap::Subcommand;
use lib::error::UdmError;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::rpc_types::fhs_types::RegulatorType;
use lib::rpc_types::service_types::AddFluidRegulatorRequest;
use lib::rpc_types::service_types::CollectFluidRegulatorsRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::ModifyFluidRegulatorRequest;
use lib::rpc_types::service_types::RemoveFluidRegulatorRequest;
use lib::UdmResult;
use tonic::async_trait;

#[derive(Subcommand, Debug)]
pub enum FluidCommands {
    #[command(about = "Add a fluid regulator")]
    Add(AddFluidArgs),
    #[command(about = "Show current fluid regulators")]
    Show(ShowFluidArgs),
    #[command(about = "Remove a fluid regulator")]
    Remove(RemoveFluidArgs),
    #[command(about = "Update a fluid regulator")]
    Update(UpdateFluidArgs),
}
#[async_trait]
impl MainCommandHandler for FluidCommands {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        match self {
            // Need to pass the client information here
            FluidCommands::Add(user_input) => user_input.handle_command(options).await,
            FluidCommands::Show(user_input) => user_input.handle_command(options).await,
            FluidCommands::Remove(user_input) => user_input.handle_command(options).await,
            FluidCommands::Update(user_input) => user_input.handle_command(options).await,
        }
    }
}

#[derive(Args, Debug)]
pub struct AddFluidArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short, long, help = "Specify the ID")]
    fr_id: Option<i32>,
    #[arg(short='t', long="type", help="Type of regulator", value_parser=RegulatorType::get_possible_values())]
    reg_type: Option<String>,
    #[arg(
        short = 'g',
        long = "gpio_pin",
        help = "The GPIO pin the device is connected"
    )]
    gpio_pin: Option<i32>,
}
impl UdmGrpcActions<FluidRegulator> for AddFluidArgs {
    fn sanatize_input(&self) -> UdmResult<FluidRegulator> {
        if let Some(raw_input) = &self.raw {
            log::debug!("Json passed: {}", &raw_input);
            let fluid: FluidRegulator = serde_json::from_str(raw_input)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            fluid.validate_without_id_fields()?;
            return Ok(fluid);
        }
        if self.reg_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(FluidRegulator {
            fr_id: self.fr_id,
            regulator_type: Some(
                RegulatorType::from_str_name(self.reg_type.clone().unwrap().as_str())
                    .unwrap()
                    .into(),
            ),
            gpio_pin: self.gpio_pin,
        })
    }
}
#[async_trait]
impl MainCommandHandler for AddFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fr = self.sanatize_input().unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .add_fluid_regulator(AddFluidRegulatorRequest { fluid: Some(fr) })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        log::info!(
            "Inserted into database, got ID back {}",
            response.into_inner().fr_id
        );
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct UpdateFluidArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: Option<String>,
    #[arg(short, long, help = "Specify the ID")]
    fr_id: Option<i32>,
    #[arg(short='t', long="type", help="Type of regulator", value_parser=RegulatorType::get_possible_values())]
    reg_type: Option<String>,
    #[arg(
        short = 'g',
        long = "gpio_pin",
        help = "The GPIO pin the device is connected"
    )]
    gpio_pin: Option<i32>,
}
impl UdmGrpcActions<FluidRegulator> for UpdateFluidArgs {
    fn sanatize_input(&self) -> UdmResult<FluidRegulator> {
        if let Some(raw_input) = &self.raw {
            log::debug!("Json passed: {}", &raw_input);
            let fluid: FluidRegulator = serde_json::from_str(raw_input)
                .map_err(|_| UdmError::InvalidInput(String::from("Failed to parse json")))?;
            fluid.validate_all_fields()?;
            return Ok(fluid);
        }
        if self.fr_id.is_none() || self.reg_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(FluidRegulator {
            fr_id: self.fr_id,
            regulator_type: Some(
                RegulatorType::from_str_name(self.reg_type.clone().unwrap().as_str())
                    .unwrap()
                    .into(),
            ),
            gpio_pin: self.gpio_pin,
        })
    }
}
#[async_trait]
impl MainCommandHandler for UpdateFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fr = self.sanatize_input().unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(2)
        });
        let mut open_connection = options.connect().await?;
        let response = open_connection
            .update_fluid_regulator(ModifyFluidRegulatorRequest { fluid: Some(fr) })
            .await
            .map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        log::info!(
            "Updated database, got ID back {}",
            response.into_inner().fr_id
        );
        Ok(())
    }
}
#[derive(Args, Debug)]
pub struct ShowFluidArgs {
    query_options: String
}
#[async_trait]
impl MainCommandHandler for ShowFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let fetched = self.sanatize_input()?;
        let mut open_connection = options.connect().await?;
        let response = open_connection.collect_fluid_regulators(CollectFluidRegulatorsRequest{expressions: fetched}).await.map_err(|e| UdmError::ApiFailure(format!("{}", e)))?;
        log::debug!("Got response {:?}", response);
        for data in response.into_inner().fluids {
            println!("{:?}", data);
        }
        Ok(())
    }
}
impl UdmGrpcActions<Vec<FetchData>> for ShowFluidArgs {
    fn sanatize_input(&self) -> UdmResult<Vec<FetchData>> {
        let wheres: Vec<&str> = self.query_options.split(",").collect();
        let mut collected_queries: Vec<FetchData> = Vec::new();
        for clause in wheres {
            let sanatized_data = FetchData::to_fetch_data(clause)?;
            collected_queries.push(sanatized_data);
        }
        Ok(collected_queries)
    }
}
#[derive(Args, Debug)]
pub struct RemoveFluidArgs {
    #[arg(short, long, help = "Remove fluid regulator by ID", required = true)]
    fr_id: Option<i32>,
}
#[async_trait]
impl MainCommandHandler for RemoveFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        let id = self.fr_id.ok_or_else(|| {
            UdmError::InvalidInput("Invalid input to remove fluid regulator".to_string())
        })?;
        let _ = ensure_removal();
        let req = RemoveFluidRegulatorRequest { fr_id: id };
        let mut open_conn = options.connect().await?;
        let response = open_conn.remove_fluid_regulator(req).await;
        log::debug!("Got response {:?}", response);
        match response {
            Ok(_) => {
                log::info!("Successfully removed from database");
            }
            Err(err) => {
                log::error!("Error removing from db: {}", err.to_string())
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::rpc_types::fhs_types::FluidRegulator;
    use lib::rpc_types::fhs_types::RegulatorType;

    #[test]
    fn test_sanatize_add_input() {
        let add_fluid = AddFluidArgs {
            raw: None,
            fr_id: None,
            reg_type: Some("REGULATOR_TYPE_VALVE".to_string()),
            gpio_pin: Some(12),
        };
        let fr = add_fluid.sanatize_input();
        let expected_result = FluidRegulator {
            regulator_type: Some(RegulatorType::Valve.into()),
            gpio_pin: Some(12),
            ..Default::default()
        };
        assert_eq!(fr.unwrap(), expected_result)
    }
    #[test]
    fn test_sanatize_input_add_raw() {
        let raw = r#"{"fr_id": 1, "regulator_type": 1, "gpio_pin": 12}"#.to_string();
        let add_fluid = AddFluidArgs {
            raw: Some(raw),
            fr_id: None,
            reg_type: None,
            gpio_pin: None,
        };
        let fr = add_fluid.sanatize_input();
        let expected_result = FluidRegulator {
            fr_id: Some(1),
            regulator_type: Some(RegulatorType::Valve.into()),
            gpio_pin: Some(12),
            ..Default::default()
        };
        assert_eq!(fr.unwrap(), expected_result)
    }
    #[test]
    fn test_sanatize_update_input() {
        let update_fluid = UpdateFluidArgs {
            raw: None,
            fr_id: Some(1),
            reg_type: Some("REGULATOR_TYPE_VALVE".to_string()),
            gpio_pin: Some(12),
        };
        let fr: UdmResult<FluidRegulator> = update_fluid.sanatize_input();
        let expected_result = FluidRegulator {
            fr_id: Some(1),
            regulator_type: Some(RegulatorType::Valve.into()),
            gpio_pin: Some(12),
        };
        assert_eq!(fr.unwrap(), expected_result)
    }
    #[test]
    fn test_sanatize_input_update_raw() {
        let raw = r#"{"fr_id": 1, "regulator_type": 1, "gpio_pin": 12}"#.to_string();
        let update_fluid = UpdateFluidArgs {
            raw: Some(raw),
            fr_id: None,
            reg_type: None,
            gpio_pin: None,
        };
        let fr: UdmResult<FluidRegulator> = update_fluid.sanatize_input();
        let expected_result = FluidRegulator {
            fr_id: Some(1),
            regulator_type: Some(RegulatorType::Valve.into()),
            gpio_pin: Some(12),
            ..Default::default()
        };
        assert_eq!(fr.unwrap(), expected_result)
    }
    #[test]
    fn test_sanatize_input_add_raw_not_all_valued() {
        let raw = r#"{"gpio_pin": 12}"#.to_string();
        let add_fluid = AddFluidArgs {
            raw: Some(raw),
            fr_id: None,
            reg_type: None,
            gpio_pin: None,
        };
        let fr = add_fluid.sanatize_input();
        assert_eq!(
            fr.unwrap_err().to_string(),
            "Invalid Input `Not all required fields were passed`".to_string()
        )
    }
    #[test]
    fn test_sanatize_add_input_not_all_values() {
        let add_fluid = AddFluidArgs {
            raw: None,
            fr_id: None,
            reg_type: None,
            gpio_pin: Some(12),
        };
        let fr = add_fluid.sanatize_input();
        assert_eq!(fr.is_err(), true)
    }
    #[test]
    fn test_sanatize_input_update_raw_not_all_valued() {
        let raw = r#"{"regulator_type": 1, "gpio_pin": 12}"#.to_string();
        let update_fluid = UpdateFluidArgs {
            raw: Some(raw),
            fr_id: None,
            reg_type: None,
            gpio_pin: None,
        };
        let fr: UdmResult<FluidRegulator> = update_fluid.sanatize_input();
        assert_eq!(
            fr.unwrap_err().to_string(),
            "Invalid Input `Not all required fields were passed`".to_string()
        )
    }
    #[test]
    fn test_sanatize_update_input_not_all_values() {
        let update_fluid = UpdateFluidArgs {
            raw: None,
            fr_id: None,
            reg_type: Some("REGULATOR_TYPE_VALVE".to_string()),
            gpio_pin: Some(12),
        };
        let fr: UdmResult<FluidRegulator> = update_fluid.sanatize_input();
        assert_eq!(fr.is_err(), true)
    }
}
