use env_logger::filter::{Builder, Filter};
use log::{Log, Metadata, Record, SetLoggerError};

const FILTER_ENV: &str = "RUST_LOG";

pub struct MyLogger {
    pub inner: Filter,
}

impl MyLogger {
    pub fn new() -> MyLogger {
        let mut builder = Builder::from_env(FILTER_ENV);
        MyLogger {
            inner: builder.build(),
        }
    }
    pub fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}
impl Default for MyLogger {
    fn default() -> Self {
        Self::new()
    }
}
impl Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }
    // TODO: Set Colors To Specific Levels
    fn log(&self, record: &Record) {
        // Check if the record is matched by the logger before logging
        if self.inner.matches(record) {
            println!("[{}:{}] {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}