use postgres::Error as PostgresError;
use rusqlite::Error as rusqlite_error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UdmError {
    #[error("Invalid Configuration {0}")]
    InvalidateConfiguration(String),
    #[error("An Error from Sqlite")]
    RusqliteError(#[from] rusqlite_error),
    #[error("An Error from Postgres")]
    PostgresError(#[from] PostgresError),
}

impl From<String> for UdmError {
    fn from(value: String) -> Self {
        Self::InvalidateConfiguration(value)
    }
}
