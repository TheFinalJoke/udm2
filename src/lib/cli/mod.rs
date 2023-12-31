use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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

    #[arg(long, help = "Run in Daemon mode", default_value = "true")]
    pub daemon: bool,

    #[command(subcommand)]
    command: Option<UdmCommands>,
}
impl Default for Cli {
    fn default() -> Self {
        Self {
            debug: 0,
            config_file: Path::new("/etc/udm/default.toml").to_path_buf(),
            daemon: true,
            command: None,
        }
    }
}

impl Cli {
    pub fn new(debug: u8, config_file: &str, daemon: bool) -> Self {
        Self {
            debug: debug,
            config_file: Path::new(config_file).to_path_buf(),
            daemon: daemon,
            command: None,
        }
    }
}

#[derive(Subcommand, Debug)]
enum UdmCommands {
    #[command(about = "To interact with recipes")]
    Recipe(RecipeCommands),
    #[command(about = "To interact with ingredients")]
    Ingredient(IngredientCommands),
    #[command(about = "To interact with instructions")]
    Instruction(InstructionCommands),
    #[command(about = "To interact with fluid")]
    Fluid(FluidCommands),
}

#[derive(Args, Debug)]
struct RecipeCommands {
    #[arg(short, long)]
    recipe_id: Option<i64>,
}

#[derive(Args, Debug)]
struct IngredientCommands {
    #[arg(short, long)]
    ingredient_id: Option<i64>,
}

#[derive(Args, Debug)]
struct InstructionCommands {
    #[arg(short, long)]
    instruction_id: Option<i64>,
}

#[derive(Args, Debug)]
struct FluidCommands {
    #[arg(short, long)]
    fr_id: Option<i64>,
}
