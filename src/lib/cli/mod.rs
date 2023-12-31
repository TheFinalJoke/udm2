use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, help="Turn on debugging, the more (d)s more verbose")]
    pub debug: u8,

    #[arg(short, long, value_name = "FILE", help = "Path to Config File")]
    pub config_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<UdmCommands>,
}

#[derive(Subcommand, Debug)]
enum UdmCommands {
    #[command(about = "Initiaize database and factory resets")]
    Init(InitCommands),
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
struct InitCommands{}

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
