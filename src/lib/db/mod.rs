use std::fmt::Display;
use tokio_postgres::Row;
use tonic::async_trait;

use crate::error::UdmError;
use crate::parsers::settings;
use crate::rpc_types::fhs_types::RegulatorType;
use crate::rpc_types::MultipleValues;
use crate::UdmResult;
use sea_query::foreign_key::ForeignKeyAction;
use sea_query::foreign_key::ForeignKeyCreateStatement;
use sea_query::value::Value;
use sea_query::ColumnDef;
use sea_query::Iden;
use sea_query::Table;
use std::sync::Arc;
pub mod executor;
pub mod postgres;
pub mod sqlite;

// Build "loadable" different db types with their relevant information

// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactionsFactory: Display {
    fn column_to_str(&self) -> &'static str;
    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized;
}

// This generates schemas and manupulates tables outside of the data itself
// This ipml on each individual table you want to
pub trait SqlTableTransactionsFactory: SqlTransactionsFactory {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String;
    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String;
    fn truncate_table<T: sea_query::Iden + 'static>(
        table: T,
        builder: impl sea_query::backend::SchemaBuilder,
    ) -> String {
        Table::truncate().table(table).to_owned().to_string(builder)
    }
}

// This Generates and executes the actual queries
#[async_trait]
pub trait DatabaseTransactionsFactory {
    async fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>>;
    async fn gen_schmea(&mut self) -> UdmResult<()>;
    async fn truncate_schema(&self) -> UdmResult<()>;
}

#[async_trait]
pub trait DbConnection: DatabaseTransactionsFactory + Send + Sync {
    // Documentation for datatypes: https://docs.rs/postgres/0.14.0/postgres/types/trait.FromSql.html#types
    async fn insert(&self, stmt: String) -> UdmResult<i32>;
    async fn delete(&self, stmt: String) -> UdmResult<()>;
    async fn update(&self, stmt: String) -> UdmResult<i32>;
    async fn select(&self, stmt: String) -> UdmResult<Vec<Row>>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DbMetaData {
    pub dbtype: Arc<DbType>,
}

impl DbMetaData {
    pub fn new(dbtype: Arc<DbType>) -> Self {
        Self { dbtype }
    }
}

// A loadable enum depending on the mechanism is chosen
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DbType {
    Postgres(settings::PostgresConfigurer),
    Sqlite(settings::SqliteConfigurer),
}
impl DbType {
    pub fn load_db(udm_configurer: Arc<settings::UdmConfigurer>) -> Self {
        if let Some(postgres_configurer) = udm_configurer.daemon.postgres.clone() {
            tracing::info!("Using postgres as the Database");
            Self::Postgres(postgres_configurer)
        } else if let Some(sqlite_configurer) = udm_configurer.daemon.sqlite.clone() {
            tracing::info!("Using sqlite as the database");
            Self::Sqlite(sqlite_configurer)
        } else {
            panic!("Could not determine database to use and load")
        }
    }
    pub async fn establish_connection(&self) -> Box<dyn DbConnection> {
        match self {
            DbType::Postgres(config) => {
                Box::new(postgres::conn::OpenPostgresConnection::new(config.to_owned()).await)
            }
            DbType::Sqlite(config) => {
                Box::new(sqlite::conn::OpenSqliteConnection::new(config.to_owned()).await)
            }
        }
    }
}

// Defines the Schema and how we interact with the DB.
// The structs generated in RPC Frameworks
// We will Transform different types
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "FluidRegulation"]
pub enum FluidRegulationSchema {
    Table,
    FrId, // Primary Key
    GpioPin,
    RegulatorType,
}
impl SqlTransactionsFactory for FluidRegulationSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "FluidRegulation",
            Self::FrId => "fr_id",
            Self::GpioPin => "gpio_pin",
            Self::RegulatorType => "regulator_type",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "FluidRegulation" => Some(FluidRegulationSchema::Table),
            "fr_id" => Some(FluidRegulationSchema::FrId),
            "gpio_pin" => Some(FluidRegulationSchema::GpioPin),
            "regulator_type" => Some(FluidRegulationSchema::RegulatorType),
            _ => None,
        }
    }
}

impl Display for FluidRegulationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valid Fields are:\n\
        fr_id: int\n\
        gpio_pin: int\n\
        regulator_type: {:?}
        ",
            RegulatorType::get_possible_values()
        )
    }
}
impl SqlTableTransactionsFactory for FluidRegulationSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::FrId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::RegulatorType).integer().not_null())
            .col(ColumnDef::new(Self::GpioPin).integer())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

impl TryFrom<String> for FluidRegulationSchema {
    type Error = UdmError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FluidRegulation" => Ok(FluidRegulationSchema::Table),
            "fr_id" => Ok(FluidRegulationSchema::FrId),
            "gpio_pin" => Ok(FluidRegulationSchema::GpioPin),
            "regulator_type" => Ok(FluidRegulationSchema::RegulatorType),
            _ => Err(UdmError::ApiFailure("Failed to collect Column".to_string())),
        }
    }
}
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Ingredient"]
pub enum IngredientSchema {
    Table,
    IngredientId,
    Name,
    Alcoholic,
    Description,
    IsActive,
    FrId, // Foreign Key
    Amount,
    IngredientType,
    InstructionId, // Foriegn Key
}
impl SqlTransactionsFactory for IngredientSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Ingredient",
            Self::IngredientId => "ingredient_id",
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
        match value {
            "Ingredient" => Some(Self::Table),
            "ingredient_id" => Some(Self::IngredientId),
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
impl Display for IngredientSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valid Fields are:\n\
        ingredient_id: int\n\
        name: string\n\
        alcoholic: bool\n\
        description: string\n\
        is_active: bool\n\
        amount: int\n\
        ingredient_type: int\n\
        fr_id: int\n\
        instruction_id: int\n\
        "
        )
    }
}
impl TryFrom<String> for IngredientSchema {
    type Error = UdmError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Ingredient" => Ok(Self::Table),
            "ingredient_id" => Ok(Self::IngredientId),
            "name" => Ok(Self::Name),
            "alcoholic" => Ok(Self::Alcoholic),
            "description" => Ok(Self::Description),
            "is_active" => Ok(Self::IsActive),
            "amount" => Ok(Self::Amount),
            "ingredient_type" => Ok(Self::IngredientType),
            "fr_id" => Ok(Self::FrId),
            "instruction_id" => Ok(Self::InstructionId),
            _ => Err(UdmError::ApiFailure(
                "Failed to collect IngredientSchema Column".to_string(),
            )),
        }
    }
}

impl SqlTableTransactionsFactory for IngredientSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::IngredientId)
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
            .col(ColumnDef::new(Self::FrId).integer())
            .col(ColumnDef::new(Self::InstructionId).integer())
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_fluidregulation")
                    .from(Self::Table, Self::FrId)
                    .to(FluidRegulationSchema::Table, FluidRegulationSchema::FrId)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_instruction")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::InstructionId)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::SetNull),
            )
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Instruction"]
pub enum InstructionSchema {
    Table,
    InstructionId,
    InstructionDetail,
    InstructionName,
}
impl SqlTransactionsFactory for InstructionSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Instruction",
            Self::InstructionId => "instruction_id",
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
            "instruction_id" => Some(Self::InstructionId),
            "instruction_detail" => Some(Self::InstructionDetail),
            "instruction_name" => Some(Self::InstructionName),
            _ => None,
        }
    }
}
impl Display for InstructionSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valid Fields are:\n\
        instruction_id: int\n\
        instruction_detail: int\n\
        instruction_name: int\n\
        "
        )
    }
}
impl TryFrom<String> for InstructionSchema {
    type Error = UdmError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "instruction" => Ok(Self::Table),
            "instruction_id" => Ok(Self::InstructionId),
            "instruction_detail" => Ok(Self::InstructionDetail),
            "instruction_name" => Ok(Self::InstructionName),
            _ => Err(UdmError::ApiFailure(
                "Failed to collect InstructionSchema column".to_string(),
            )),
        }
    }
}

impl SqlTableTransactionsFactory for InstructionSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::InstructionId)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::InstructionDetail).text())
            .col(ColumnDef::new(Self::InstructionName).text().not_null())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[derive(Iden, Eq, PartialEq, Debug)]
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
        match value {
            "InstructionToRecipe" => Some(Self::Table),
            "recipe_id" => Some(Self::RecipeId),
            "instruction_id" => Some(Self::InstructionId),
            "instruction_order" => Some(Self::InstructionOrder),
            _ => None,
        }
    }
}
impl Display for InstructionToRecipeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valid Fields are:\n\
        recipe_id: int\n\
        instruction_id: int\n\
        instruction_order: int\n\
        "
        )
    }
}
impl TryFrom<String> for InstructionToRecipeSchema {
    type Error = UdmError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "InstructionToRecipe" => Ok(Self::Table),
            "recipe_id" => Ok(Self::RecipeId),
            "instruction_id" => Ok(Self::InstructionId),
            "instruction_order" => Ok(Self::InstructionOrder),
            _ => Err(UdmError::ApiFailure(
                "Failed to collect InstructionToRecipeSchema Column".to_string(),
            )),
        }
    }
}
impl SqlTableTransactionsFactory for InstructionToRecipeSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(ColumnDef::new(Self::RecipeId).integer())
            .col(ColumnDef::new(Self::InstructionId).integer())
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_recipe")
                    .from(Self::Table, Self::RecipeId)
                    .to(RecipeSchema::Table, RecipeSchema::RecipeId)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk_instruction")
                    .from(Self::Table, Self::InstructionId)
                    .to(InstructionSchema::Table, InstructionSchema::InstructionId)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::SetNull),
            )
            .col(ColumnDef::new(Self::InstructionOrder).integer().not_null())
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}
#[derive(Iden, Eq, PartialEq, Debug)]
#[iden = "Recipe"]
pub enum RecipeSchema {
    Table,
    RecipeId,
    Name,
    UserInput,
    DrinkSize,
    Description,
}
impl SqlTransactionsFactory for RecipeSchema {
    fn column_to_str(&self) -> &'static str {
        match self {
            Self::Table => "Recipe",
            Self::RecipeId => "recipe_id",
            Self::Name => "name",
            Self::UserInput => "user_input",
            Self::DrinkSize => "drink_size",
            Self::Description => "description",
        }
    }
    fn from_str(value: &'static str) -> Option<Self> {
        match value {
            "Recipe" => Some(Self::Table),
            "recipe_id" => Some(Self::RecipeId),
            "name" => Some(Self::Name),
            "user_input" => Some(Self::UserInput),
            "drink_size" => Some(Self::DrinkSize),
            "description" => Some(Self::Description),
            _ => None,
        }
    }
}
impl Display for RecipeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Valid Fields are:\n\
        recipe_id: int\n\
        name: string\n\
        user_input: bool\n\
        drink_size: int\n\
        description: string\n\
        "
        )
    }
}
impl TryFrom<String> for RecipeSchema {
    type Error = UdmError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Recipe" => Ok(Self::Table),
            "recipe_id" => Ok(Self::RecipeId),
            "name" => Ok(Self::Name),
            "user_input" => Ok(Self::UserInput),
            "drink_size" => Ok(Self::DrinkSize),
            "description" => Ok(Self::Description),
            _ => Ok(Self::Table),
        }
    }
}
impl SqlTableTransactionsFactory for RecipeSchema {
    fn create_table(builder: impl sea_query::backend::SchemaBuilder) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::RecipeId)
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
            .build(builder)
    }

    fn alter_table(
        builder: impl sea_query::backend::SchemaBuilder,
        column_def: &mut ColumnDef,
    ) -> String {
        Table::alter()
            .table(Self::Table)
            .add_column(column_def)
            .build(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_to_str() {
        let fr = FluidRegulationSchema::GpioPin;
        assert_eq!(fr.column_to_str(), "gpio_pin")
    }

    #[test]
    fn str_to_column() {
        let fr_str = "gpio_pin";
        assert_eq!(
            FluidRegulationSchema::from_str(fr_str),
            Some(FluidRegulationSchema::GpioPin)
        )
    }
}
