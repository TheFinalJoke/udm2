use std::fmt;

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
enum FluidRegulationSchema {
    Id = 0, // Primary Key
    GpioPin = 1,
    RegulatorType = 2,
}

enum IngredientSchema {
    Name = 0,
    Alcoholic = 1,
    Description = 2,
    IsActive = 3,
    FrId = 4, // Foreign Key
    Amount = 5,
    IngredientType = 6,
    InstructionId = 7, // Foriegn Key
}
enum InstructionSchema {
    Id = 0,
    InstructionDetail = 1,
    InstructionName = 2,
}
enum InstructionToRecipeSchema {
    RecipeId = 1,      // Forigen Key
    InstructionId = 2, // Forigen Key
    InstructionOrder = 3,
}

enum RecipeSchema {
    Id = 0,
    Name = 1,
    UserInput = 2,
    DrinkSize = 3,
    Description = 4,
}
// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactions {
    fn add(&self) -> String;
    fn modify(&self) -> String;
    fn drop(&self) -> String;
    fn get(&self) -> String;
}
