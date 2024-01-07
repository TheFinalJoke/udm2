use lib::db::{sqlite, SqlTableTransactionsFactory};

#[test]
fn fluid_regulation_table_create() {
    let sql_query = [
        r#"CREATE TABLE IF NOT EXISTS "FluidRegulation""#,
        r#"( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""regulator_type" integer NOT NULL, "gpio_pin" integer )"#,
    ]
    .join(" ");

    assert_eq!(
        sqlite::FluidRegulationSchema::create_table().to_string(),
        sql_query
    );
}

#[test]
fn fluid_regulation_table_alter() {
    let query = r#"ALTER TABLE "FluidRegulation" ADD COLUMN "gpio_pin" integer"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::FluidRegulationSchema::GpioPin);
    let binding = binding.integer();
    assert_eq!(sqlite::FluidRegulationSchema::alter_table(binding), query);
}

#[test]
fn ingredient_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Ingredient""#,
        r#"( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""name" text NOT NULL,"#,
        r#""alcoholic" boolean NOT NULL DEFAULT FALSE,"#,
        r#""description" text,"#,
        r#""is_active" boolean NOT NULL DEFAULT FALSE,"#,
        r#""amount" real,"#,
        r#""ingredient_type" integer NOT NULL,"#,
        r#"FOREIGN KEY ("fr_id") REFERENCES "FluidRegulation""#,
        r#"("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
        r#"FOREIGN KEY ("instruction_id") REFERENCES "Instruction""#,
        r#"("id") ON DELETE CASCADE ON UPDATE CASCADE )"#,
    ]
    .join(" ");
    assert_eq!(sqlite::IngredientSchema::create_table().to_string(), query);
}

#[test]
fn ingredient_table_alter() {
    let query = r#"ALTER TABLE "Ingredient" ADD COLUMN "description" text"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::IngredientSchema::Description);
    let binding = binding.text();
    assert_eq!(sqlite::IngredientSchema::alter_table(binding), query);
}

#[test]
fn instruction_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Instruction""#,
        r#"( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""instruction_detail" text,"#,
        r#""instruction_name" text NOT NULL )"#,
    ]
    .join(" ");
    assert_eq!(sqlite::InstructionSchema::create_table().to_string(), query);
}

#[test]
fn instruction_table_alter() {
    let query = r#"ALTER TABLE "Instruction" ADD COLUMN "instruction_name" text"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::InstructionSchema::InstructionName);
    let binding = binding.text();
    assert_eq!(sqlite::InstructionSchema::alter_table(binding), query);
}

#[test]
fn instruction_to_recipe_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "InstructionToRecipe""#,
        r#"( "instruction_order" integer NOT NULL,"#,
        r#"FOREIGN KEY ("recipe_id") REFERENCES "Recipe" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
        r#"FOREIGN KEY ("instruction_id") REFERENCES "Instruction" ("id") ON DELETE CASCADE ON UPDATE CASCADE )"#
    ].join(" ");
    assert_eq!(
        sqlite::InstructionToRecipeSchema::create_table().to_string(),
        query
    );
}

#[test]
fn instruction_to_recipe_alter() {
    let query = r#"ALTER TABLE "InstructionToRecipe" ADD COLUMN "recipe_id" text"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::InstructionToRecipeSchema::RecipeId);
    let binding = binding.text();
    assert_eq!(
        sqlite::InstructionToRecipeSchema::alter_table(binding),
        query
    );
}

#[test]
fn recipe_table_create() {
    let query = [
        r#"CREATE TABLE IF NOT EXISTS "Recipe""#,
        r#"( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""name" text NOT NULL UNIQUE,"#,
        r#""user_input" boolean NOT NULL DEFAULT FALSE,"#,
        r#""drink_size" integer NOT NULL DEFAULT 0,"#,
        r#""description" text NOT NULL UNIQUE )"#,
    ]
    .join(" ");
    assert_eq!(sqlite::RecipeSchema::create_table().to_string(), query);
}

#[test]
fn recipe_alter() {
    let query = r#"ALTER TABLE "Recipe" ADD COLUMN "name" text"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::RecipeSchema::Name);
    let binding = binding.text();
    assert_eq!(sqlite::RecipeSchema::alter_table(binding), query);
}
