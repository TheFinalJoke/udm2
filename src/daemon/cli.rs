use clap::Parser;
use clap::Subcommand;
use clap_verbosity_flag::Verbosity;
use std::path::Path;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DaemonCli {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

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
            verbose: Verbosity::default(),
            config_file: Path::new("/etc/udm/default.toml").to_path_buf(),
            command: None,
        }
    }
}

impl DaemonCli {
    pub fn new(config_file: &str) -> Self {
        Self {
            verbose: Verbosity::default(),
            config_file: Path::new(config_file).to_path_buf(),
            command: None,
        }
    }
}

#[derive(Subcommand, Debug)]
enum UdmCommands {}
