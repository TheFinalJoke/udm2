use crate::UdmResult;

use super::service_types::AddFluidRegulatorResponse;
use super::service_types::AddFluidRegulatorRequest;
use crate::db::executor::SqlQueryExecutor;
use crate::rpc_types::server::DaemonServerContext;
use crate::db::FluidRegulationSchema;
use super::fhs_types::FluidRegulator;
use super::fhs_types::RegulatorType;
use sea_query::Query;
use sea_query::InsertStatement;
use async_trait::async_trait;

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

struct FluidExecutor {
    ctx: DaemonServerContext,
    builder: Box<dyn sea_query::QueryBuilder>
}

impl FluidExecutor {
    fn insert(&self, request: AddFluidRegulatorRequest) -> tonic::Response<AddFluidRegulatorResponse> {
        todo!()
    }
}

#[async_trait]
impl SqlQueryExecutor for FluidRegulator {
    async fn insert_into_db(&self, ctx: DaemonServerContext, builder: Box<dyn sea_query::QueryBuilder>) -> UdmResult<()> {
        let query = self.gen_insert_query().to_string(builder);
        Ok(())
    }

    fn gen_insert_query(&self) -> InsertStatement {
        Query::insert()
            .into_table(FluidRegulationSchema::Table)
            .columns([FluidRegulationSchema::GpioPin, FluidRegulationSchema::RegulatorType])
            .values_panic([self.gpio_pin.into(), self.regulator_type.into()])
            .to_owned()
    }
    
}
