use sea_query::{Iden, IntoIden};
use sea_query::{Table, ColumnDef, ForeignKey, Value, backend::SqliteQueryBuilder, Query, SimpleExpr};
use std::fmt;

pub mod sqlite;

// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactions {
    fn create_table(&self) -> String;
    fn alter_table(&self, column_def: &mut ColumnDef) -> String;
    fn insert(&self) -> String;
    fn select(&self, columns: Vec<Self>) -> String;
    fn update(&self) -> String;
    fn drop(&self) -> String;
    fn drop_table(&self) -> String;
}

pub trait SqlQueryExecutor {
    fn gen_query(&self) -> Box<dyn SqlTransactions>;
    fn execute<T>(&self) ->Result<T, SqlError>;
}

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

// Defines the Schema and how we interact with the DB. 
// The structs generated in RPC Frameworks
// We will Transform different types
#[derive(Iden)]
pub enum FluidRegulationSchema {
    Table,
    Id, // Primary Key
    GpioPin,
    RegulatorType,
}

impl SqlTransactions for FluidRegulationSchema {
    fn create_table(&self) -> String {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(ColumnDef::new(Self::Id).integer().auto_increment().not_null().primary_key())
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

    fn select(&self, columns: Vec<Self>, ) -> String {
        todo!()
    }

    fn insert(&self) -> String {
        todo!()
    }

    fn update(&self) -> String {
        todo!()
    }

    fn drop(&self) -> String {
        todo!()
    }

    fn drop_table(&self) -> String {
        todo!()
    }
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
