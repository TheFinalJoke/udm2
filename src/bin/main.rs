extern crate log;
use clap::Parser;
// use lib::db::{sqlite, SqlTableTransactionsFactory};
use lib::{logger, Retrieval};
use std::error::Error;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::UdmCli::parse();
    let debug_level = logger::get_log_level(cli_opts.debug);
    logger::MyLogger::init(debug_level).unwrap();
    log::info!(
        "Initialized logger, collecting Config File {}",
        &cli_opts.config_file.display()
    );
    let config_file = lib::FileRetrieve::new(cli_opts.config_file).retreieve::<config::Config>();
    println!("{:?}", config_file);
    Ok(())
}
