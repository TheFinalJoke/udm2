use crate::cli::helpers::ensure_removal;
use crate::cli::helpers::MainCommandHandler;
use crate::cli::helpers::UdmGrpcActions;
use crate::cli::helpers::UdmServerOptions;
use clap::Args;
use clap::Subcommand;
use cli_table::Cell;
use cli_table::Style;
use cli_table::Table;
use cli_table::TableStruct;
use lib::error::UdmError;
use lib::rpc_types::fhs_types::FluidRegulator;
use lib::rpc_types::fhs_types::RegulatorType;
use lib::rpc_types::service_types::AddFluidRegulatorRequest;
use lib::rpc_types::service_types::CollectFluidRegulatorsRequest;
use lib::rpc_types::service_types::FetchData;
use lib::rpc_types::service_types::ModifyFluidRegulatorRequest;
use lib::rpc_types::service_types::RemoveFluidRegulatorRequest;
use lib::rpc_types::FieldValidation;
use lib::UdmResult;
use tonic::async_trait;

fn create_tables(data: Vec<FluidRegulator>) -> TableStruct {
    let mut table = Vec::new();
    for fluid in data {
        let fr_id = match &fluid.fr_id {
            Some(id) => format!("{}", id),
            None => "Not Set".to_string(),
        };
        let gpio_pin: String = match &fluid.gpio_pin {
            Some(pin) => format!("{}", pin),
            None => "Not Set".to_string(),
        };
        let reg = match fluid.regulator_type {
            Some(reg) => RegulatorType::try_from(reg)
                .unwrap_or(RegulatorType::Unspecified)
                .as_str_name()
                .to_string(),
            None => "Not Set".to_string(),
        };
        table.push(vec![fr_id.cell(), gpio_pin.cell(), reg.cell()]);
    }
    table
        .table()
        .title(vec![
            "ID".cell().bold(true),
            "Gpio Pin".cell().bold(true),
            "Regulator Type".cell().bold(true),
        ])
        .bold(true)
}
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
    query_options: Option<String>,
    #[arg(long, short = 'e', help = "Example queries", default_value = "false")]
    example: bool,
}
#[async_trait]
impl MainCommandHandler for ShowFluidArgs {
    async fn handle_command(&self, options: UdmServerOptions) -> UdmResult<()> {
        if self.example {
            self.show_example();
            Ok(())
        } else {
            let fetched = self.sanatize_input()?;
            let mut open_connection = options.connect().await?;
            let response = open_connection
                .collect_fluid_regulators(CollectFluidRegulatorsRequest {
                    expressions: fetched,
                })
                .await
                .map_err(|e| UdmError::ApiFailure(format!("{}", e)));
            match response {
                Ok(response) => {
                    log::debug!("Got response {:?}", &response);
                    let fluids = response.into_inner().fluids;
                    println!("Found {} results", &fluids.len());
                    let table = create_tables(fluids);
                    println!("{}", table.display().unwrap());
                    Ok(())
                }
                Err(err) => {
                    println!("Error: Could not show FRs due to: {}", err);
                    Ok(())
                }
            }
        }
    }
}
impl UdmGrpcActions<Vec<FetchData>> for ShowFluidArgs {
    fn sanatize_input(&self) -> UdmResult<Vec<FetchData>> {
        let collected_queries =
            FetchData::to_fetch_data_vec(self.query_options.clone().unwrap().as_str())?;
        Ok(collected_queries)
    }
}
impl ShowFluidArgs {
    pub(crate) fn show_example(&self) {
        println!("To build a query it will be <field><operation><values>");
        println!("fr_id=1");
        println!("^^ will query fr_id=1");
        println!("Another example of multiple values, fr_id = 1");
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
