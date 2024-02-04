use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum InstructionCommands {
    #[command(about = "Add a Instruction regulator")]
    Add(AddInstructionArgs),
    #[command(about = "Show current Instruction regulators")]
    Show(ShowInstructionArgs),
    #[command(about = "Remove current Instruction regulator")]
    Remove(RemoveInstructionArgs)
}

#[derive(Args, Debug)]
pub struct AddInstructionArgs {
    #[arg(long, value_name="JSON", help="Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value="false")]
    json: bool
}

#[derive(Args, Debug)]
pub struct ShowInstructionArgs {
    #[arg(long, value_name="JSON", help="Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value="false")]
    json: bool
}

#[derive(Args, Debug)]
pub struct RemoveInstructionArgs {
    #[arg(long, value_name="JSON", help="Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value="false")]
    json: bool
}