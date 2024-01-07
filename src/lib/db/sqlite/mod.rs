use crate::db;
use sea_query::backend::SqliteQueryBuilder;
use sea_query::foreign_key::{ForeignKeyAction, ForeignKeyCreateStatement};
use sea_query::value::Value;
use sea_query::{ColumnDef, Iden, Table};

use super::{SqlTableTransactionsFactory, SqlTransactionsFactory};

// Defines the Schema and how we interact with the DB.
// The structs generated in RPC Frameworks
// We will Transform different types
#[derive(Iden)]
#[iden = "FluidRegulation"]
pub enum FluidRegulationSchema {
    Table,
    Id, // Primary Key
    GpioPin,
    RegulatorType,
}
impl db::SqlTransactionsFactory for FluidRegulationSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "FluidRegulation",
            Self::Id => "id",
            Self::GpioPin => "gpio_pin",
            Self::RegulatorType => "regulator_type",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "fluidRegulation" => Some(FluidRegulationSchema::Table),
            "id" => Some(FluidRegulationSchema::Id),
            "gpio_pin" => Some(FluidRegulationSchema::GpioPin),
            "regulator_type" => Some(FluidRegulationSchema::RegulatorType),
            _ => None,
        }
    }
}

impl db::SqlTableTransactionsFactory for FluidRegulationSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::RegulatorType).integer().not_null())
            .col(ColumnDef::new(Self::GpioPin).integer())
            .build(SqliteQueryBuilder)
    }

    fn alter_table(&self, column_def: &mut ColumnDef) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(SqliteQueryBuilder)
    }
}
#[derive(Iden)]
#[iden = "Ingredient"]
pub enum IngredientSchema {
    Table,
    Id,
    Name,
    Alcoholic,
    Description,
    IsActive,
    FrId, // Foreign Key
    Amount,
    IngredientType,
    InstructionId, // Foriegn Key
}

#[derive(Iden)]
#[iden = "Instruction"]
pub enum InstructionSchema {
    Table,
    Id,
    InstructionDetail,
    InstructionName,
}
impl SqlTransactionsFactory for InstructionSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Instruction",
            Self::Id => "id",
            Self::InstructionDetail => "instruction_detail",
            Self::InstructionName => "instruction_name",
        }
    }

    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized,
    {
        match value.to_lowercase().as_str() {
            "instruction" => Some(Self::Table),
            "id" => Some(Self::Id),
            "instruction_detail" => Some(Self::InstructionDetail),
            "instruction_name" => Some(Self::InstructionName),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for InstructionSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::InstructionDetail).text())
            .col(ColumnDef::new(Self::InstructionName).text().not_null())
            .build(SqliteQueryBuilder)
    }

    fn alter_table(&self, column_def: &mut ColumnDef) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(SqliteQueryBuilder)
    }
}
impl db::SqlTransactionsFactory for IngredientSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Ingredient",
            Self::Id => "id",
            Self::Name => "name",
            Self::Alcoholic => "alcoholic",
            Self::Description => "description",
            Self::IsActive => "is_active",
            Self::FrId => "fr_id",
            Self::Amount => "amount",
            Self::IngredientType => "amount",
            Self::InstructionId => "instruction_id",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "ingredient" => Some(Self::Table),
            "id" => Some(Self::Id),
            "name" => Some(Self::Name),
            "alcoholic" => Some(Self::Alcoholic),
            "description" => Some(Self::Description),
            "is_active" => Some(Self::IsActive),
            "amount" => Some(Self::Amount),
            "ingredient_type" => Some(Self::IngredientType),
            "fr_id" => Some(Self::FrId),
            "instruction_id" => Some(Self::InstructionId),
            _ => None,
        }
    }
}

impl db::SqlTableTransactionsFactory for IngredientSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::Name).text().not_null())
            .col(
                ColumnDef::new(Self::Alcoholic)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(ColumnDef::new(Self::Description).text())
            .col(
                ColumnDef::new(Self::IsActive)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(ColumnDef::new(Self::Amount).float())
            .col(ColumnDef::new(Self::IngredientType).integer().not_null())
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fr_id")
                    .from(Self::Table, Self::FrId)
                    .to(FluidRegulationSchema::Table, FluidRegulationSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("instruction_id")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(SqliteQueryBuilder)
    }

    fn alter_table(&self, column_def: &mut ColumnDef) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(SqliteQueryBuilder)
    }
}
#[derive(Iden)]
#[iden = "InstructionToRecipe"]
pub enum InstructionToRecipeSchema {
    Table,
    RecipeId,
    InstructionId,
    InstructionOrder,
}
impl SqlTransactionsFactory for InstructionToRecipeSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "InstructionToRecipe",
            Self::RecipeId => "recipe_id",
            Self::InstructionId => "instruction_id",
            Self::InstructionOrder => "instruction_order",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "instructiontorecipe" => Some(Self::Table),
            "recipe_id" => Some(Self::RecipeId),
            "instruction_id" => Some(Self::InstructionId),
            "instruction_order" => Some(Self::InstructionOrder),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for InstructionToRecipeSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("recipe_id")
                    .from(Self::Table, Self::RecipeId)
                    .to(RecipeSchema::Table, RecipeSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("instruction_id")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Self::InstructionOrder).integer().not_null())
            .build(SqliteQueryBuilder)
    }

    fn alter_table(&self, column_def: &mut ColumnDef) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(SqliteQueryBuilder)
    }
}
#[derive(Iden)]
#[iden = "Recipe"]
pub enum RecipeSchema {
    Table,
    Id,
    Name,
    UserInput,
    DrinkSize,
    Description,
}
impl SqlTransactionsFactory for RecipeSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Recipe",
            Self::Id => "id",
            Self::Name => "name",
            Self::UserInput => "user_input",
            Self::DrinkSize => "drink_size",
            Self::Description => "description",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "Recipe" => Some(Self::Table),
            "id" => Some(Self::Id),
            "name" => Some(Self::Name),
            "user_input" => Some(Self::UserInput),
            "drink_size" => Some(Self::DrinkSize),
            "description" => Some(Self::Description),
            _ => None,
        }
    }
}
impl SqlTableTransactionsFactory for RecipeSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::Name).text().not_null().unique_key())
            .col(
                ColumnDef::new(Self::UserInput)
                    .boolean()
                    .not_null()
                    .default(Value::Bool(Some(false))),
            )
            .col(
                ColumnDef::new(Self::DrinkSize)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(Self::Description)
                    .text()
                    .not_null()
                    .unique_key(),
            )
            .build(SqliteQueryBuilder)
    }

    fn alter_table(&self, column_def: &mut ColumnDef) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(SqliteQueryBuilder)
    }
}
