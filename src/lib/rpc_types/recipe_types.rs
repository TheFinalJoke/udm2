tonic::include_proto!("recipe_types");

use std::fmt::Display;

use crate::db::executor::GenQueries;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::error::UdmError;
use crate::rpc_types::FieldValidation;
use crate::rpc_types::MultipleValues;
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
        if self.id == 0 || self.instruction_name.is_empty() || self.instruction_detail.is_empty() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        if self.instruction_name.is_empty() || self.instruction_detail.is_empty() {
            return Err(UdmError::InvalidInput(String::from(
                "`Not all required fields were passed`",
            )));
        }
        Ok(())
    }
}
impl FieldValidation for Ingredient {
    fn validate_all_fields(&self) -> UdmResult<()> {
        todo!();
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        todo!()
    }
}
impl FieldValidation for Recipe {
    fn validate_all_fields(&self) -> UdmResult<()> {
        todo!()
    }

    fn validate_without_id_fields(&self) -> UdmResult<()> {
        todo!()
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

impl MultipleValues for IngredientType {
    fn get_possible_values() -> Vec<&'static str> {
        [
            IngredientType::Eatables.as_str_name(),
            IngredientType::Fluid.as_str_name(),
        ]
        .to_vec()
    }
}

impl Display for IngredientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[async_trait]
impl GenQueries for Ingredient {
    fn gen_insert_query(&self) -> InsertStatement {
        Query::insert()
            .into_table(IngredientSchema::Table)
            .columns([
                IngredientSchema::Name,
                IngredientSchema::Description,
                IngredientSchema::Alcoholic,
                IngredientSchema::Amount,
                IngredientSchema::IsActive,
                IngredientSchema::IngredientType,
                IngredientSchema::FrId,
                IngredientSchema::InstructionId,
            ])
            .values_panic([
                self.name.clone().into(),
                self.description.clone().into(),
                self.is_alcoholic.into(),
                self.amount.into(),
                self.is_active.into(),
                self.ingredient_type.into(),
                self.regulator
                    .clone()
                    .map_or(0.into(), |fr| fr.fr_id.map_or(0.into(), |id| id.into())),
                self.instruction
                    .clone()
                    .map_or(0.into(), |instruction| instruction.id.into()),
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
                (IngredientSchema::Name, self.name.clone().into()),
                (
                    IngredientSchema::Description,
                    self.description.clone().into(),
                ),
                (IngredientSchema::Alcoholic, self.is_alcoholic.into()),
                (IngredientSchema::Amount, self.amount.into()),
                (IngredientSchema::IsActive, self.is_active.into()),
                (
                    IngredientSchema::IngredientType,
                    self.ingredient_type.into(),
                ),
                (
                    IngredientSchema::FrId,
                    self.regulator
                        .clone()
                        .map_or(0.into(), |fr| fr.fr_id.map_or(0.into(), |id| id.into())),
                ),
                (
                    IngredientSchema::InstructionId,
                    self.instruction
                        .clone()
                        .map_or(0.into(), |instruction| instruction.id.into()),
                ),
            ])
            .and_where(Expr::col(InstructionSchema::InstructionId).eq(self.id))
            .returning(Query::returning().column(InstructionSchema::InstructionId))
            .to_owned()
    }
}
