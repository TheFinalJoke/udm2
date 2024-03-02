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
#[cfg(test)]
mod tests {
    use super::*;

    use sea_query::PostgresQueryBuilder;

    #[test]
    fn test_gen_insert_query() {
        let fr = FluidRegulator {
            fr_id: Some(1),
            gpio_pin: Some(23),
            regulator_type: Some(RegulatorType::Tap.into()),
        };
        let query = fr.gen_insert_query().to_string(PostgresQueryBuilder);
        let expected_query = r#"INSERT INTO "FluidRegulation" ("gpio_pin", "regulator_type") VALUES (23, 3) RETURNING "fr_id""#.to_string();
        assert_eq!(query, expected_query)
    }
}
