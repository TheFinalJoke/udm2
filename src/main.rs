// use lib::rpc_types::fhs_types;
extern crate log;
use clap::Parser;
// use clap::{Arg, Command};
use lib::cli;
use lib::db::SqlTransactions;
use lib::logger;
use lib::rpc_types;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::Cli::parse();
    let debug_level = logger::get_log_level(cli_opts.debug);
    logger::MyLogger::init(debug_level).unwrap();

    let fr = rpc_types::fhs_types::FluidRegulator {
        fr_id: 1,
        gpio_pin: 2,
        regulator_type: 2,
    };
    fr.modify();
    Ok(())
}
