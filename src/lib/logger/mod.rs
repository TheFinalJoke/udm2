use env_logger::filter::{Builder, Filter};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

const FILTER_ENV: &str = "RUST_LOG";

pub struct MyLogger {
    pub inner: Filter,
}

impl MyLogger {
    pub fn new(level: Option<LevelFilter>) -> MyLogger {
        let mut build = Builder::from_env(FILTER_ENV);
        if level.is_some() | level.is_some() && level.unwrap() != LevelFilter::Off {
            build.filter_level(level.unwrap());
        }
        let logger = MyLogger {
            inner: build.build(),
        };
        logger
    }
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Self::new(Some(level));

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}
impl Default for MyLogger {
    fn default() -> Self {
        Self::new(None)
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

pub fn get_log_level(debug_cli: u8) -> LevelFilter {
    match debug_cli {
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        5 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    }
}
