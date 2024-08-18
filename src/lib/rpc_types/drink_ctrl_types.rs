use crate::db::executor::GenQueries;
use crate::db::PumpLogSchema;
use crate::rpc_types::MultipleValues;
use anyhow::Error as AnyHowError;
use async_trait::async_trait;
use postgres::row::Row;
use sea_query::DeleteStatement;
use sea_query::Expr;
use sea_query::InsertStatement;
use sea_query::Query;
use sea_query::UpdateStatement;
use sea_query::Value;
use std::fmt::Display;
use uuid::Uuid;
tonic::include_proto!("drink_ctrl_types");

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ReqType {
    Unspecified = 0,
    Dispense = 1,
    Cleaning = 2,
    GetPumpInfo = 3,
    Polling = 4,
}

impl TryFrom<i32> for ReqType {
    type Error = AnyHowError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Dispense),
            2 => Ok(Self::Cleaning),
            3 => Ok(Self::GetPumpInfo),
            4 => Ok(Self::Polling),
            _ => Ok(Self::Unspecified),
        }
    }
}
impl Display for ReqType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl MultipleValues for ReqType {
    fn get_possible_values() -> Vec<&'static str> {
        [
            ReqType::Unspecified.as_str_name(),
            ReqType::Dispense.as_str_name(),
            ReqType::Cleaning.as_str_name(),
            ReqType::GetPumpInfo.as_str_name(),
            ReqType::Polling.as_str_name(),
        ]
        .to_vec()
    }
}
impl ReqType {
    pub fn to_i32(&self) -> i32 {
        match self {
            ReqType::Unspecified => 0,
            ReqType::Dispense => 1,
            ReqType::Cleaning => 2,
            ReqType::GetPumpInfo => 3,
            ReqType::Polling => 4,
        }
    }
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReqType::Unspecified => "Unspecified",
            ReqType::Dispense => "Dispense",
            ReqType::Cleaning => "Cleaning",
            ReqType::GetPumpInfo => "GetPumpInfo",
            ReqType::Polling => "Polling",
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct PumpLogger {
    pub(crate) req_id: Uuid,
    pub(crate) req_type: ReqType,
    pub(crate) fluid_id: Option<i32>,
}
impl PumpLogger {
    pub(crate) fn new(req_id: Option<Uuid>, req_type: ReqType, fluid_id: Option<i32>) -> Self {
        let req_id = req_id.unwrap_or(Uuid::new_v4());
        Self {
            req_id,
            req_type,
            fluid_id,
        }
    }
}

#[async_trait]
impl GenQueries for PumpLogger {
    fn gen_insert_query(&self) -> InsertStatement {
        Query::insert()
            .into_table(PumpLogSchema::Table)
            .columns([
                PumpLogSchema::ReqId,
                PumpLogSchema::ReqType,
                PumpLogSchema::FluidId,
            ])
            .values_panic([
                self.req_id.into(),
                self.req_type.to_i32().into(),
                self.fluid_id.into(),
            ])
            .returning(Query::returning().column(PumpLogSchema::ReqId))
            .to_owned()
    }
    fn gen_remove_query(_id: i32) -> DeleteStatement {
        unimplemented!()
    }
    fn gen_custom_remove_query(&self) -> DeleteStatement {
        Query::delete()
            .from_table(PumpLogSchema::Table)
            .and_where(Expr::col(PumpLogSchema::ReqId).eq(Value::Uuid(Some(Box::new(self.req_id)))))
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        Query::update()
            .table(PumpLogSchema::Table)
            .values([
                (PumpLogSchema::ReqType, self.req_type.to_i32().into()),
                (PumpLogSchema::FluidId, self.fluid_id.into()),
            ])
            .and_where(Expr::col(PumpLogSchema::ReqId).eq(Value::Uuid(Some(Box::new(self.req_id)))))
            .returning(Query::returning().column(PumpLogSchema::ReqId))
            .to_owned()
    }
}

impl TryFrom<Row> for PumpLogger {
    type Error = AnyHowError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            req_id: Uuid::parse_str(value.try_get(0)?)?,
            req_type: ReqType::try_from(value.try_get::<usize, i32>(1)?)?,
            fluid_id: value.try_get(2)?,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::executor::GenQueries;
    use sea_query::PostgresQueryBuilder;
    #[test]
    fn test_select_pump_logger() {
        let uuid = Uuid::parse_str("827a1328-f763-412e-92ba-f31e2111a4eb").unwrap();
        let pl = PumpLogger::new(Some(uuid), ReqType::Dispense, None);
        let result = pl.gen_insert_query().to_string(PostgresQueryBuilder);
        let expected = r#"INSERT INTO "Pumplog" ("req_id", "req_type", "fluid_id") VALUES ('827a1328-f763-412e-92ba-f31e2111a4eb', 1, NULL) RETURNING "req_id""#;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_update_pump_logger() {
        let uuid = Uuid::parse_str("827a1328-f763-412e-92ba-f31e2111a4eb").unwrap();
        let pl = PumpLogger::new(Some(uuid), ReqType::Dispense, None);
        let result = pl.gen_update_query().to_string(PostgresQueryBuilder);
        let expected = r#"UPDATE "Pumplog" SET "req_type" = 1, "fluid_id" = NULL WHERE "req_id" = '827a1328-f763-412e-92ba-f31e2111a4eb' RETURNING "req_id""#;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_removal_pump_logger() {
        let uuid = Uuid::parse_str("827a1328-f763-412e-92ba-f31e2111a4eb").unwrap();
        let pl = PumpLogger::new(Some(uuid), ReqType::Dispense, None);
        let result = pl.gen_custom_remove_query().to_string(PostgresQueryBuilder);
        let expected =
            r#"DELETE FROM "Pumplog" WHERE "req_id" = '827a1328-f763-412e-92ba-f31e2111a4eb'"#;
        assert_eq!(result, expected);
    }
}
