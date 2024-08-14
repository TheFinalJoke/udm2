// Note: The Module must be the same name as the package in proto

use std::fmt::Display;

use crate::UdmResult;
use std::fmt::Debug;
pub mod drink_controller;
pub mod drink_ctrl_types;
pub mod fhs_types;
pub mod gpio_types;
pub mod recipe_types;
pub mod server;
pub mod service_types;
use crate::rpc_types::fhs_types::FluidRegulator;
use crate::rpc_types::recipe_types::Ingredient;
use crate::rpc_types::recipe_types::Instruction;
use crate::rpc_types::recipe_types::Recipe;
pub trait FieldValidation {
    fn validate_all_fields(&self) -> UdmResult<()>;
    fn validate_without_id_fields(&self) -> UdmResult<()>;
}

pub trait MultipleValues: Display {
    fn get_possible_values() -> Vec<&'static str>;
}

/// This is for "main structures" for easy validation and creation
pub trait ProtoGen: Debug + FieldValidation {}

impl ProtoGen for FluidRegulator {}
impl ProtoGen for Instruction {}
impl ProtoGen for Ingredient {}
impl ProtoGen for Recipe {}
