use lib::db::{sqlite, SqlTableTransactionsFactory};

#[test]
fn fluid_regulation_table_create() {
    let sql_query = [
        r#"CREATE TABLE IF NOT EXISTS "FluidRegulation""#,
        r#"( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
        r#""regulator_type" integer NOT NULL, "gpio_pin" integer )"#,
    ]
    .join(" ");

    assert_eq!(sqlite::FluidRegulationSchema::create_table().to_string(), sql_query);
}

#[test]
fn fluid_regulation_table_alter() {
    let query = r#"ALTER TABLE "FluidRegulation" ADD COLUMN "gpio_pin" integer"#;
    let mut binding = sea_query::ColumnDef::new(sqlite::FluidRegulationSchema::GpioPin);
    let binding = binding.integer();
    assert_eq!(sqlite::FluidRegulationSchema::alter_table(binding), query);
}