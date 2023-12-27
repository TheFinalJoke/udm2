// use lib::rpc_types::fhs_types;
extern crate log;
use clap::Parser;
// use clap::{Arg, Command};
use lib::logger::MyLogger;
use lib::cli;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::Cli::parse();
    // Create Logger
    MyLogger::init().unwrap();
    
    Ok(())
}
