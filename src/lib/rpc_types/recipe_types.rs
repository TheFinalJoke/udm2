tonic::include_proto!("recipe_types");

use crate::db::executor::GenQueries;
use crate::db::InstructionSchema;
use crate::error::UdmError;
use crate::rpc_types::FieldValidation;
use crate::UdmResult;
use anyhow::Error as AnyError;
use async_trait::async_trait;
use postgres::row::Row;
use sea_query::DeleteStatement;
use sea_query::Expr;
use sea_query::InsertStatement;
use sea_query::Query;
use sea_query::UpdateStatement;

impl FieldValidation for Instruction {
    fn validate_all_fields(&self) -> UdmResult<()> {
        if self.id.is_none() || self.instruction_name.is_none() || self.instruction_detail.is_none()
        {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        if self.instruction_name.is_none() || self.instruction_detail.is_none() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl GenQueries for Instruction {
    fn gen_insert_query(&self) -> InsertStatement {
        Query::insert()
            .into_table(InstructionSchema::Table)
            .columns([
                InstructionSchema::InstructionName,
                InstructionSchema::InstructionDetail,
            ])
            .values_panic([
                self.instruction_name.clone().into(),
                self.instruction_name.clone().into(),
            ])
            .returning(Query::returning().column(InstructionSchema::InstructionId))
            .to_owned()
    }
    fn gen_remove_query(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(InstructionSchema::Table)
            .and_where(Expr::col(InstructionSchema::InstructionId).eq(id))
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        Query::update()
            .table(InstructionSchema::Table)
            .values([
                (
                    InstructionSchema::InstructionName,
                    self.instruction_name.clone().into(),
                ),
                (
                    InstructionSchema::InstructionDetail,
                    self.instruction_detail.clone().into(),
                ),
            ])
            .and_where(Expr::col(InstructionSchema::InstructionId).eq(self.id))
            .returning(Query::returning().column(InstructionSchema::InstructionId))
            .to_owned()
    }
}
impl TryFrom<Row> for Instruction {
    type Error = AnyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get(0)?,
            instruction_detail: value.try_get(1)?,
            instruction_name: value.try_get(2)?,
        })
    }
}

