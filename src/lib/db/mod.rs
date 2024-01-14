use log;
use rusqlite::Connection;
use sea_query::ColumnDef;
use core::panic;
use std::fmt;

use self::sqlite::FluidRegulationSchema;
use self::sqlite::IngredientSchema;
use self::sqlite::InstructionSchema;
use self::sqlite::InstructionToRecipeSchema;
use self::sqlite::RecipeSchema;
pub mod sqlite;
pub mod postgres;

// Build "loadable" different db types with their relevant information

// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactionsFactory {
    fn column_to_str(&self) -> &'static str;
    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized;
}
pub trait SqlTableTransactionsFactory: SqlTransactionsFactory {
    fn create_table() -> String;
    fn alter_table(column_def: &mut ColumnDef) -> String;
}
pub(crate) struct NonRelatedTable;

impl NonRelatedTable {
    #[allow(dead_code)]
    fn collect_all_current_tables(open_conn: &Connection) -> rusqlite::Result<Vec<String>> {
        log::debug!("Getting current tables in db");
        let mut stmt =
            open_conn.prepare("SELECT name FROM main.sqlite_master WHERE type='table'")?;
        let table_rows = stmt.query_map([], |rows| rows.get(0))?;
        let mut tables: Vec<String> = Vec::new();
        for row in table_rows {
            tables.push(row?);
        }
        log::trace!("Data: tables {:?}", tables);
        Ok(tables)
    }
    fn gen_schmea(open_conn: &Connection) -> rusqlite::Result<()> {
        let tables = [
            FluidRegulationSchema::create_table(),
            InstructionSchema::create_table(),
            RecipeSchema::create_table(),
            IngredientSchema::create_table(),
            InstructionToRecipeSchema::create_table(),
        ]
        .join("; ");
        log::debug!("Ensure schmea is defined and exists");
        let mut batch = rusqlite::Batch::new(open_conn, &tables);
        while let Some(mut stmt) = batch.next().unwrap_or_else(|e| panic!("Failure creating the schema {:?} with query {:?}", e, batch)) {
            log::trace!("Using Batched query {:?}", stmt);
            stmt.execute([]).unwrap_or_else(|e| panic!("Failure creating the schema {:?} with query {:?}", e, stmt.expanded_sql()));
        };
        log::debug!("Tables created");
        Ok(())
    }
}
pub trait SqlRowTransactionsFactory: SqlTransactionsFactory {}
pub trait SqlQueryExecutor {
    fn gen_query(&self) -> Box<dyn SqlTransactionsFactory>;
    fn execute<T>(&self) -> Result<T, SqlError>;
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
