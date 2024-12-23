use regex::Error as RegexError;
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
    #[error("Invalid Input {0}")]
    InvalidInput(String),
    #[error("Api Failure: {0}")]
    ApiFailure(String),
    #[error("Error Parsing: {0}")]
    ParsingError(#[from] RegexError),
    #[error("Error Setting Up Logger: {0}")]
    LoggerError(String),
    #[error("Error collecting GpioPin: {0}")]
    GpioError(String),
}

impl From<String> for UdmError {
    fn from(value: String) -> Self {
        Self::InvalidateConfiguration(value)
    }
}

impl UdmError {
    pub fn log_and_exit(msg: Box<dyn GenericError>, exit_code: i32) {
        tracing::error!("{}", format!("{}", msg));
        std::process::exit(exit_code)
    }
}
