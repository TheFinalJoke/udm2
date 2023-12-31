use std::fmt;
use sea_query::Iden;
use crate::cli::Cli;
use std::rc::Rc;

pub mod sqlite;

#[derive(Debug)]
pub enum SqlError {
    QueryError,
}
impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::QueryError => write!(f, "A query error as occured"),
        }
    }
}
// Defines the Schema
#[derive(Iden)]
pub enum FluidRegulationSchema {
    Table,
    Id, // Primary Key
    GpioPin,
    RegulatorType,
}
#[derive(Iden)]
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
pub enum InstructionSchema {
    Table,
    Id,
    InstructionDetail,
    InstructionName,
}
#[derive(Iden)]
pub enum InstructionToRecipeSchema {
    Table,
    RecipeId,      // Forigen Key
    InstructionId, // Forigen Key
    InstructionOrder,
}
#[derive(Iden)]
pub enum RecipeSchema {
    Table,
    Id,
    Name,
    UserInput,
    DrinkSize,
    Description,
}
// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactions {
    fn add(&self) -> String;
    fn modify(&self) -> String;
    fn drop(&self) -> String;
    fn get(&self) -> String;
}

pub fn create_or_check_database() -> Result<(), rusqlite::Error> {

    Ok(())
}