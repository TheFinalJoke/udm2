extern crate log;
use clap::Parser;

use lib::{logger, parsers, Retrieval};
use std::error::Error;
use std::rc::Rc;

// use sea_query::Iden;
// use lib::rpc_types;

pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_opts = cli::DaemonCli::parse();
    let debug_level = logger::get_log_level(cli_opts.debug);
    logger::MyLogger::init(debug_level).unwrap();
    log::info!(
        "Initialized logger, collecting Config File {}",
        &cli_opts.config_file.display()
    );
    let config_file = lib::FileRetrieve::new(cli_opts.config_file).retreieve::<config::Config>()?;
    
    let configeror = Rc::new(
        config_file.try_deserialize::<parsers::settings::UdmConfigurer>()?,
    );
    println!("{:?}", configeror);
    // let fr = rpc_types::fhs_types::FluidRegulator {
    //     fr_id: 1,
    //     gpio_pin: 2,
    //     regulator_type: 2,
    // };
    // fr.modify();
    Ok(())
}
