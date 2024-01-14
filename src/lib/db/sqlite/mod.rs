use core::panic;
use log;

use super::NonRelatedTableFactory;
use crate::db;
use crate::db::SqlTableTransactionsFactory;
use crate::UdmResult;

use rusqlite::Connection;
use sea_query::backend::SqliteQueryBuilder;

pub mod conn;

#[derive(Debug)]
pub struct NonRelatedTableSqlite<'a> {
    open_conn: &'a Connection,
}

impl<'a> NonRelatedTableFactory for NonRelatedTableSqlite<'a> {
    #[allow(dead_code)]
    fn collect_all_current_tables(&self) -> UdmResult<Vec<String>> {
        log::debug!("Getting current tables in db");
        let mut stmt = self
            .open_conn
            .prepare("SELECT name FROM main.sqlite_master WHERE type='table'")?;
        let table_rows = stmt.query_map([], |rows| rows.get(0))?;
        let mut tables: Vec<String> = Vec::new();
        for row in table_rows {
            tables.push(row?);
        }
        log::trace!("Data: tables {:?}", tables);
        Ok(tables)
    }
    #[allow(dead_code)]
    fn gen_schmea(&self) -> UdmResult<()> {
        let tables = [
            db::FluidRegulationSchema::create_table(SqliteQueryBuilder),
            db::InstructionSchema::create_table(SqliteQueryBuilder),
            db::RecipeSchema::create_table(SqliteQueryBuilder),
            db::IngredientSchema::create_table(SqliteQueryBuilder),
            db::InstructionToRecipeSchema::create_table(SqliteQueryBuilder),
        ]
        .join("; ");
        log::debug!("Ensure schmea is defined and exists");
        let mut batch = rusqlite::Batch::new(self.open_conn, &tables);
        while let Some(mut stmt) = batch.next().unwrap_or_else(|e| {
            panic!("Failure creating the schema {:?} with query {:?}", e, batch)
        }) {
            log::trace!("Using Batched query {:?}", stmt);
            stmt.execute([]).unwrap_or_else(|e| {
                panic!(
                    "Failure creating the schema {:?} with query {:?}",
                    e,
                    stmt.expanded_sql()
                )
            });
        }
        log::debug!("Tables created");
        Ok(())
    }
}
