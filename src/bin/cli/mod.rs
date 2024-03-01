use clap::Parser;
use clap::Subcommand;
pub mod fluid;
pub mod helpers;
pub mod ingredient;
pub mod instruction;
pub mod recipe;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct UdmCli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, help="Turn on debugging, the more (d)s more verbose")]
    pub debug: u8,

    #[arg(
        short = 's',
        long,
        help = "Ip for udm_server",
        default_value = "127.0.0.1"
    )]
    pub udm_server: std::net::Ipv4Addr,
    #[arg(
        short = 'p',
        long,
        help = "Connect to udm port",
        default_value = "19211"
    )]
    pub udm_port: i64,
    #[command(subcommand)]
    pub command: Option<UdmCommand>,
}
impl Default for UdmCli {
    fn default() -> Self {
        Self {
            debug: 0,
            udm_server: std::net::Ipv4Addr::new(127, 0, 0, 1),
            udm_port: 19211,
            command: None,
        }
    }
}

impl UdmCli {
    pub fn new(debug: u8) -> Self {
        Self {
            debug,
            udm_server: std::net::Ipv4Addr::new(127, 0, 0, 1),
            udm_port: 19211,
            command: None,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum UdmCommand {
    #[command(about = "To interact with recipes", subcommand)]
    Recipe(recipe::RecipeCommands),
    #[command(about = "To interact with ingredients", subcommand)]
    Ingredient(ingredient::IngredientCommands),
    #[command(about = "To interact with instructions", subcommand)]
    Instruction(instruction::InstructionCommands),
    #[command(about = "To interact with fluid", subcommand)]
    Fluid(fluid::FluidCommands),
}
