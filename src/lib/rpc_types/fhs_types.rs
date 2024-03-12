use crate::db::executor::GenQueries;
use crate::db::FluidRegulationSchema;
use crate::error::UdmError;
use crate::UdmResult;
use anyhow::Error as AnyError;
use async_trait::async_trait;
use postgres::row::Row;
use sea_query::DeleteStatement;
use sea_query::Expr;
use sea_query::InsertStatement;
use sea_query::Query;
use sea_query::UpdateStatement;


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
    fn gen_remove_query(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(FluidRegulationSchema::Table)
            .and_where(Expr::col(FluidRegulationSchema::FrId).eq(id))
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        Query::update()
            .table(FluidRegulationSchema::Table)
            .values([
                (FluidRegulationSchema::GpioPin, self.gpio_pin.into()),
                (
                    FluidRegulationSchema::RegulatorType,
                    self.regulator_type.into(),
                ),
            ])
            .and_where(Expr::col(FluidRegulationSchema::FrId).eq(self.fr_id))
            .returning(Query::returning().column(FluidRegulationSchema::FrId))
            .to_owned()
    }
}

impl FluidRegulator {
    pub fn validate_all_fields(&self) -> UdmResult<()> {
        if self.fr_id.is_none() || self.regulator_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }
    pub fn validate_without_id_fields(&self) -> UdmResult<()> {
        if self.regulator_type.is_none() || self.gpio_pin.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }
}
impl TryFrom<Row> for FluidRegulator {
    type Error = AnyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            fr_id: value.try_get(0)?,
            regulator_type: value.try_get(1)?,
            gpio_pin: value.try_get(2)?,
        })
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

    #[test]
    fn test_gen_remove_query() {
        let fr_id = 2;
        let query = FluidRegulator::gen_remove_query(fr_id);
        let expected = r#"DELETE FROM "FluidRegulation" WHERE "fr_id" = 2"#;
        assert_eq!(query.to_string(PostgresQueryBuilder), expected);
    }

    #[test]
    fn test_gen_update_query() {
        let fr = FluidRegulator {
            fr_id: Some(1),
            gpio_pin: Some(23),
            regulator_type: Some(RegulatorType::Tap.into()),
        };
        let query = fr.gen_update_query().to_string(PostgresQueryBuilder);
        let expected_query = r#"UPDATE "FluidRegulation" SET "gpio_pin" = 23, "regulator_type" = 3 WHERE "fr_id" = 1 RETURNING "fr_id""#.to_string();
        assert_eq!(query, expected_query)
    }
}
