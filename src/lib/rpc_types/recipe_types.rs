tonic::include_proto!("recipe_types");

use std::collections::HashMap;
use std::fmt::Display;

use crate::db::executor::GenQueries;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::db::InstructionToRecipeSchema;
use crate::db::RecipeSchema;
use crate::error::UdmError;
use crate::rpc_types::service_types::InstructionToRecipeMetadata;
use crate::rpc_types::FieldValidation;
use crate::rpc_types::FluidRegulator;
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
impl TryFrom<Row> for Ingredient {
    type Error = AnyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get(0)?,
            name: value.try_get(1)?,
            is_alcoholic: value.try_get(2)?,
            description: value.try_get(3)?,
            is_active: value.try_get(4)?,
            amount: value.try_get(5)?,
            ingredient_type: value.try_get(6)?,
            regulator: {
                value.try_get(7).map_or(None, |f| {
                    Some(FluidRegulator {
                        fr_id: f,
                        gpio_pin: None,
                        regulator_type: None,
                    })
                })
            },
            instruction: {
                value.try_get(8).map_or(None, |id: Option<i32>| {
                    Some(Instruction {
                        id: id.unwrap_or_default(),
                        instruction_detail: "".to_string(),
                        instruction_name: "".to_string(),
                    })
                })
            },
        })
    }
}
impl MultipleValues for IngredientType {
    fn get_possible_values() -> Vec<&'static str> {
        [
            IngredientType::Unspecified.as_str_name(),
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
        let mut columns = vec![
            IngredientSchema::Name,
            IngredientSchema::Description,
            IngredientSchema::Alcoholic,
            IngredientSchema::Amount,
            IngredientSchema::IsActive,
            IngredientSchema::IngredientType,
        ];
        let mut values = vec![
            self.name.clone().into(),
            self.description.clone().into(),
            self.is_alcoholic.into(),
            self.amount.into(),
            self.is_active.into(),
            self.ingredient_type.into(),
        ];
        if let Some(fr) = self.regulator {
            if let Some(id) = fr.fr_id {
                columns.push(IngredientSchema::FrId);
                values.push(id.into());
            }
        }
        if let Some(instruction) = self.instruction.clone() {
            columns.push(IngredientSchema::InstructionId);
            values.push(instruction.id.into());
        }
        Query::insert()
            .into_table(IngredientSchema::Table)
            .columns(columns)
            .values_panic(values)
            .returning(Query::returning().column(IngredientSchema::IngredientId))
            .to_owned()
    }
    fn gen_remove_query(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(IngredientSchema::Table)
            .and_where(Expr::col(IngredientSchema::IngredientId).eq(id))
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        let mut values = vec![
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
        ];
        if let Some(fr) = self.regulator {
            if let Some(id) = fr.fr_id {
                values.push((IngredientSchema::FrId, id.into()));
            }
        }
        if let Some(instruction) = self.instruction.clone() {
            values.push((IngredientSchema::InstructionId, instruction.id.into()));
        }
        Query::update()
            .table(IngredientSchema::Table)
            .values(values)
            .and_where(Expr::col(IngredientSchema::IngredientId).eq(self.id))
            .returning(Query::returning().column(IngredientSchema::IngredientId))
            .to_owned()
    }
}
impl TryFrom<Row> for Recipe {
    type Error = AnyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get(0)?,
            name: value.try_get(1)?,
            user_input: value.try_get(2)?,
            size: value.try_get(3)?,
            description: value.try_get(4)?,
            instructions: HashMap::new(),
        })
    }
}
impl MultipleValues for DrinkSize {
    fn get_possible_values() -> Vec<&'static str> {
        [
            DrinkSize::Unspecified.as_str_name(),
            DrinkSize::Small.as_str_name(),
            DrinkSize::Medium.as_str_name(),
            DrinkSize::Pint.as_str_name(),
            DrinkSize::Large.as_str_name(),
            DrinkSize::ExtraLarge.as_str_name(),
        ]
        .to_vec()
    }
}

impl Display for DrinkSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[async_trait]
impl GenQueries for Recipe {
    fn gen_insert_query(&self) -> InsertStatement {
        let columns = vec![
            RecipeSchema::Name,
            RecipeSchema::UserInput,
            RecipeSchema::DrinkSize,
            RecipeSchema::Description,
        ];
        let values = vec![
            self.name.clone().into(),
            self.user_input.into(),
            self.size.into(),
            self.description.clone().into(),
        ];
        Query::insert()
            .into_table(RecipeSchema::Table)
            .columns(columns)
            .values_panic(values)
            .returning(Query::returning().column(RecipeSchema::RecipeId))
            .to_owned()
    }
    fn gen_remove_query(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(RecipeSchema::Table)
            .and_where(Expr::col(RecipeSchema::RecipeId).eq(id))
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        let values = vec![
            (RecipeSchema::Name, self.name.clone().into()),
            (RecipeSchema::Description, self.description.clone().into()),
            (RecipeSchema::UserInput, self.user_input.into()),
            (RecipeSchema::DrinkSize, self.size.into()),
        ];
        Query::update()
            .table(RecipeSchema::Table)
            .values(values)
            .and_where(Expr::col(RecipeSchema::RecipeId).eq(self.id))
            .returning(Query::returning().column(RecipeSchema::RecipeId))
            .to_owned()
    }
}
impl TryFrom<Row> for InstructionToRecipeMetadata {
    type Error = AnyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get(0)?,
            recipe_id: value.try_get(1)?,
            instruction_id: value.try_get(2)?,
            instruction_order: value.try_get(3)?,
        })
    }
}
#[async_trait]
impl GenQueries for InstructionToRecipeMetadata {
    fn gen_insert_query(&self) -> InsertStatement {
        let columns = vec![
            InstructionToRecipeSchema::RecipeId,
            InstructionToRecipeSchema::InstructionId,
            InstructionToRecipeSchema::InstructionOrder,
        ];
        let values = vec![
            self.recipe_id.into(),
            self.instruction_id.into(),
            self.instruction_order.into(),
        ];
        Query::insert()
            .into_table(InstructionToRecipeSchema::Table)
            .columns(columns)
            .values_panic(values)
            .returning(Query::returning().column(InstructionToRecipeSchema::Id))
            .to_owned()
    }
    fn gen_remove_query(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(InstructionToRecipeSchema::Table)
            .and_where(Expr::col(InstructionToRecipeSchema::Id).eq(id))
            .to_owned()
    }
    fn gen_custom_remove_query(&self) -> DeleteStatement {
        Query::delete()
            .from_table(InstructionToRecipeSchema::Table)
            .and_where(Expr::col(InstructionToRecipeSchema::RecipeId).eq(self.recipe_id))
            .and_where(Expr::col(InstructionToRecipeSchema::InstructionId).eq(self.instruction_id))
            .and_where(
                Expr::col(InstructionToRecipeSchema::InstructionOrder).eq(self.instruction_order),
            )
            .to_owned()
    }
    fn gen_update_query(&self) -> UpdateStatement {
        let values = vec![
            (InstructionToRecipeSchema::RecipeId, self.recipe_id.into()),
            (
                InstructionToRecipeSchema::InstructionId,
                self.instruction_id.into(),
            ),
            (
                InstructionToRecipeSchema::InstructionOrder,
                self.instruction_order.into(),
            ),
        ];
        Query::update()
            .table(RecipeSchema::Table)
            .values(values)
            .and_where(Expr::col(InstructionToRecipeSchema::RecipeId).eq(self.recipe_id))
            .to_owned()
    }
}
