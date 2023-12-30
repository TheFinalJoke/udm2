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
// This represents the table operations itself.
// Connection and Manipulation will be handled into a different struct
pub trait SqlTransactions {
    fn add(&self) -> String;
    fn modify(&self) -> String;
    fn drop(&self) -> String;
    fn get(&self) -> String;
}
