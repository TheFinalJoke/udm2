use crate::db::SqlTableTransactionsFactory;
use lib::db;
use sea_query::backend::SqliteQueryBuilder;
#[test]
fn fluid_regulation_table_create() {
    let sql_query = [
        r#"CREATE TABLE IF NOT EXISTS "FluidRegulation""#,
        r#"( "fr_id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""regulator_type" integer NOT NULL, "gpio_pin" integer )"#,
    ]
    .join(" ");

    assert_eq!(
        db::FluidRegulationSchema::create_table(SqliteQueryBuilder).to_string(),
        sql_query
    );
}

#[test]
fn fluid_regulation_table_alter() {
    let query = r#"ALTER TABLE "FluidRegulation" ADD COLUMN "gpio_pin" integer"#;
    let mut binding = sea_query::ColumnDef::new(db::FluidRegulationSchema::GpioPin);
    let binding = binding.integer();
    assert_eq!(
        db::FluidRegulationSchema::alter_table(SqliteQueryBuilder, binding),
        query
    );
}

#[test]
fn ingredient_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Ingredient""#,
        r#"( "ingredient_id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""name" text NOT NULL,"#,
        r#""alcoholic" boolean NOT NULL DEFAULT FALSE,"#,
        r#""description" text,"#,
        r#""is_active" boolean NOT NULL DEFAULT FALSE,"#,
        r#""amount" real,"#,
        r#""ingredient_type" integer NOT NULL,"#,
        r#"FOREIGN KEY ("fr_id") REFERENCES "FluidRegulation""#,
        r#"("fr_id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
        r#"FOREIGN KEY ("instruction_id") REFERENCES "Instruction""#,
        r#"("instruction_id") ON DELETE CASCADE ON UPDATE CASCADE )"#,
    ]
    .join(" ");
    assert_eq!(
        db::IngredientSchema::create_table(SqliteQueryBuilder).to_string(),
        query
    );
}

#[test]
fn ingredient_table_alter() {
    let query = r#"ALTER TABLE "Ingredient" ADD COLUMN "description" text"#;
    let mut binding = sea_query::ColumnDef::new(db::IngredientSchema::Description);
    let binding = binding.text();
    assert_eq!(
        db::IngredientSchema::alter_table(SqliteQueryBuilder, binding),
        query
    );
}

#[test]
fn instruction_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Instruction""#,
        r#"( "instruction_id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""instruction_detail" text,"#,
        r#""instruction_name" text NOT NULL )"#,
    ]
    .join(" ");
    assert_eq!(
        db::InstructionSchema::create_table(SqliteQueryBuilder).to_string(),
        query
    );
}

#[test]
fn instruction_table_alter() {
    let query = r#"ALTER TABLE "Instruction" ADD COLUMN "instruction_name" text"#;
    let mut binding = sea_query::ColumnDef::new(db::InstructionSchema::InstructionName);
    let binding = binding.text();
    assert_eq!(
        db::InstructionSchema::alter_table(SqliteQueryBuilder, binding),
        query
    );
}

#[test]
fn instruction_to_recipe_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "InstructionToRecipe""#,
        r#"( "instruction_order" integer NOT NULL,"#,
        r#"FOREIGN KEY ("recipe_id") REFERENCES "Recipe" ("recipe_id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
        r#"FOREIGN KEY ("instruction_id") REFERENCES "Instruction" ("instruction_id") ON DELETE CASCADE ON UPDATE CASCADE )"#
    ].join(" ");
    assert_eq!(
        db::InstructionToRecipeSchema::create_table(SqliteQueryBuilder).to_string(),
        query
    );
}

#[test]
fn instruction_to_recipe_alter() {
    let query = r#"ALTER TABLE "InstructionToRecipe" ADD COLUMN "recipe_id" text"#;
    let mut binding = sea_query::ColumnDef::new(db::InstructionToRecipeSchema::RecipeId);
    let binding = binding.text();
    assert_eq!(
        db::InstructionToRecipeSchema::alter_table(SqliteQueryBuilder, binding),
        query
    );
}

#[test]
fn recipe_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Recipe""#,
        r#"( "recipe_id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""name" text NOT NULL UNIQUE,"#,
        r#""user_input" boolean NOT NULL DEFAULT FALSE,"#,
        r#""drink_size" integer NOT NULL DEFAULT 0,"#,
        r#""description" text NOT NULL UNIQUE )"#,
    ]
    .join(" ");
    assert_eq!(
        db::RecipeSchema::create_table(SqliteQueryBuilder).to_string(),
        query
    );
}

#[test]
fn recipe_alter() {
    let query = r#"ALTER TABLE "Recipe" ADD COLUMN "name" text"#;
    let mut binding = sea_query::ColumnDef::new(db::RecipeSchema::Name);
    let binding = binding.text();
    assert_eq!(
        db::RecipeSchema::alter_table(SqliteQueryBuilder, binding),
        query
    );
}
