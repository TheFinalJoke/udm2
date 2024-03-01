use crate::db::executor::GenQueries;
use crate::db::FluidRegulationSchema;
use async_trait::async_trait;
use sea_query::InsertStatement;
use sea_query::Query;

tonic::include_proto!("fhs_types");

impl RegulatorType {
    pub fn get_possible_values() -> Vec<&'static str> {
        [
            RegulatorType::Pump.as_str_name(),
            RegulatorType::Tap.as_str_name(),
            RegulatorType::Valve.as_str_name(),
        ]
        .to_vec()
    }
}

#[async_trait]
impl GenQueries for FluidRegulator {
    fn gen_insert_query(&self) -> InsertStatement {
        Query::insert()
            .into_table(FluidRegulationSchema::Table)
            .columns([
                FluidRegulationSchema::GpioPin,
                FluidRegulationSchema::RegulatorType,
            ])
            .values_panic([self.gpio_pin.into(), self.regulator_type.into()])
            .returning(Query::returning().column(FluidRegulationSchema::FrId))
            .to_owned()
    }
}
