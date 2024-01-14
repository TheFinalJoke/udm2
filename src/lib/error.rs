use rusqlite::Error as rusqlite_error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UdmError {
    #[error("Invalid Configuration {0}")]
    InvalidateConfiguration(String),
    #[error("An Error from Sqlite")]
    RusqliteError(#[from] rusqlite_error),
}

impl From<String> for UdmError {
    fn from(value: String) -> Self {
        Self::InvalidateConfiguration(value)
    }
}
