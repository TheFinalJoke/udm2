use crate::error::UdmError;
use crate::UdmResult;
use clap_verbosity_flag::Verbosity;
use std::env::var;
use std::fs::File;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

fn convert_log_level_to_tracing_level(level: log::Level) -> tracing::Level {
    match level {
        log::Level::Error => Level::ERROR,
        log::Level::Warn => Level::WARN,
        log::Level::Info => Level::INFO,
        log::Level::Debug => Level::DEBUG,
        log::Level::Trace => Level::TRACE,
    }
}

/// Descibes the different situations we need to account for
/// Daemon mode requires logging to file and structured format
/// ** But also requires pretty format for debug sessions on cli
/// Cli mode should only log to stderr
/// ** But log when user asks for it
pub enum UdmLoggerType {
    DAEMON,
    BIN,
}

pub struct UdmLogger;

impl UdmLogger {
    // writes to a file
    pub fn init(
        logger_type: UdmLoggerType,
        verbose: Verbosity,
        log_file_path: Option<&str>,
        is_test: bool,
    ) -> UdmResult<()> {
        let mut layers = Vec::new();
        // type of mode we are in will configure the layer
        match logger_type {
            UdmLoggerType::DAEMON => {
                // Log always to file
                // optionally get "nice logs" when debug

                // Create log to file layer
                if log_file_path.is_none() {
                    return Err(UdmError::LoggerError(
                        "Daemon requires a log path".to_string(),
                    ));
                }
                let file = File::create(log_file_path.unwrap_or("/var/log/udm/udm_daemon.log"))
                    .map_err(|e| UdmError::LoggerError(e.to_string()))?;
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .with_line_number(true)
                        .with_target(true)
                        .with_writer(file)
                        .with_filter(LevelFilter::from_level(convert_log_level_to_tracing_level(
                            verbose.log_level().unwrap(),
                        )))
                        .boxed(),
                );

                // Create "pretty" Layer for stdout
                // If RUST_LOG or DEBUG or cli test
                if var("RUST_LOG").is_ok() || var("DEBUG").is_ok() || is_test {
                    layers.push(
                        tracing_subscriber::fmt::layer()
                            .pretty()
                            .with_file(false)
                            .with_target(false)
                            .with_filter(LevelFilter::DEBUG)
                            .boxed(),
                    )
                }

                tracing_subscriber::registry().with(layers).init();
                Ok(())
            }
            UdmLoggerType::BIN => {
                let cli_layer = tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_file(false)
                    .compact()
                    .with_filter(LevelFilter::from_level(convert_log_level_to_tracing_level(
                        verbose.log_level().unwrap(),
                    )));
                tracing_subscriber::registry().with(cli_layer).init();
                Ok(())
            }
        }
    }
}
