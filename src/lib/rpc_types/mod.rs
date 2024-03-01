// Note: The Module must be the same name as the package in proto

pub mod fhs_types;
pub mod recipe_types;
pub mod server;
pub mod service_types;

pub enum TaskIdentifier {
    AddFluid,
    ModFluid,
    RemoveFluid,
    AddRecipe,
    GetRecipe,
    ModRecipe,
    AddInstruction,
    GetInstruction,
    ModInstruction,
    AddIngredient,
    RemoveIngredient,
    GetIngredient,
}
