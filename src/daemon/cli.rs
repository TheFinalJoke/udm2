use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DaemonCli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, help="Turn on debugging, the more (d)s more verbose")]
    pub debug: u8,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to Config File",
        default_value = "/etc/udm/default.toml"
    )]
    pub config_file: PathBuf,

    #[command(subcommand)]
    command: Option<UdmCommands>,
}
impl Default for DaemonCli {
    fn default() -> Self {
        Self {
            debug: 0,
            config_file: Path::new("/etc/udm/default.toml").to_path_buf(),
            command: None,
        }
    }
}

impl DaemonCli {
    pub fn new(debug: u8, config_file: &str) -> Self {
        Self {
            debug: debug,
            config_file: Path::new(config_file).to_path_buf(),
            command: None,
        }
    }
}

#[derive(Subcommand, Debug)]
enum UdmCommands {}