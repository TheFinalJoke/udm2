use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, help="Turn on debugging, the more (d)s more verbose")]
    debug: u8,
    
    #[arg(short, long, value_name = "FILE", help="Path to Config File")]
    config_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<UdmCommands>,

}

#[derive(Subcommand)]
enum UdmCommands{
    #[command(about="To interact with recipes")]
    Recipe(RecipeCommands),
    #[command(about="To interact with ingredients")]
    Ingredient(IngredientCommands),
    #[command(about="To interact with instructions")]
    Instruction(InstructionCommands),
    #[command(about="To interact with fluid")]
    Fluid(FluidCommands),
}


#[derive(Args)]
struct RecipeCommands {
    #[arg(short, long)]
    recipe_id: Option<i64>
}

#[derive(Args)]
struct IngredientCommands {
    #[arg(short, long)]
    ingredient_id: Option<i64>
}

#[derive(Args)]
struct InstructionCommands {
    #[arg(short, long)]
    instruction_id: Option<i64>
}

#[derive(Args)]
struct FluidCommands {
    #[arg(short, long)]
    fr_id: Option<i64>
}
