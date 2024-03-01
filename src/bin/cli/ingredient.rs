use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum IngredientCommands {
    #[command(about = "Add a Ingredient")]
    Add(AddIngredientArgs),
    #[command(about = "Show current Ingredient")]
    Show(ShowIngredientArgs),
    #[command(about = "Remove current Ingredient ")]
    Remove(RemoveIngredientArgs),
}

#[derive(Args, Debug)]
pub struct AddIngredientArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Args, Debug)]
pub struct ShowIngredientArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}

#[derive(Args, Debug)]
pub struct RemoveIngredientArgs {
    #[arg(long, value_name = "JSON", help = "Raw json to transform")]
    raw: String,
    #[arg(short, long)]
    fr_id: Option<i64>,
    #[arg(short, long, default_value = "false")]
    json: bool,
}
