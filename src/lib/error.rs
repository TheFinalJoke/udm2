use rusqlite::Error as rusqlite_error;
use std::error::Error as GenericError;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;
#[derive(Error, Debug)]
pub enum UdmError {
    #[error("Invalid Configuration {0}")]
    InvalidateConfiguration(String),
    #[error("An Error from Sqlite")]
    RusqliteError(#[from] rusqlite_error),
    #[error("An Error from Postgres {0}")]
    PostgresError(#[from] PostgresError),
}

impl From<String> for UdmError {
    fn from(value: String) -> Self {
        Self::InvalidateConfiguration(value)
    }
}

impl UdmError {
    pub fn log_and_exit(msg: Box<dyn GenericError>, exit_code: i32) {
        log::error!("{}", format!("{}", msg));
        std::process::exit(exit_code)
    }
}
