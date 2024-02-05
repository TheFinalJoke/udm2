use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum RecipeCommands {
    #[command(about = "Add a Recipe")]
    Add(AddRecipeArgs),
    #[command(about = "Show current Recipes")]
    Show(ShowRecipeArgs),
    #[command(about = "Remove current a recipe")]
    Remove(RemoveRecipeArgs),
}

#[derive(Args, Debug)]
pub struct AddRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Args, Debug)]
pub struct ShowRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Args, Debug)]
pub struct RemoveRecipeArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}
