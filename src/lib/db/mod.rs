use sea_query::ColumnDef;
use std::fmt;
pub mod sqlite;

// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactionsFactory {
    fn column_to_str(&self) -> &'static str;
    fn from_str(value: &'static str) -> Option<Self>
    where
        Self: Sized;
}
pub trait SqlTableTransactionsFactory: SqlTransactionsFactory {
    fn create_table(&self) -> String;
    fn alter_table(&self, column_def: &mut ColumnDef) -> String;
}

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
