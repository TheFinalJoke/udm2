use core::panic;
use log;

use super::DatabaseTransactionsFactory;
use crate::db;
use crate::db::SqlTableTransactionsFactory;
use crate::UdmResult;

use sea_query::backend::SqliteQueryBuilder;
use std::fmt::Display;

pub mod conn;

pub struct DatabaseTransactionSqlite<'a> {
    open_conn: &'a conn::OpenSqliteConnection,
}

impl<'a> DatabaseTransactionsFactory for DatabaseTransactionSqlite<'a> {
    #[allow(dead_code)]
    fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>> {
        log::debug!("Getting current tables in db");
        let mut stmt = self
            .open_conn.connection
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
    fn gen_schmea(&mut self) -> UdmResult<()> {
        let tables = [
            db::FluidRegulationSchema::create_table(SqliteQueryBuilder),
            db::InstructionSchema::create_table(SqliteQueryBuilder),
            db::RecipeSchema::create_table(SqliteQueryBuilder),
            db::IngredientSchema::create_table(SqliteQueryBuilder),
            db::InstructionToRecipeSchema::create_table(SqliteQueryBuilder),
        ]
        .join("; ");
        log::debug!("Ensure schmea is defined and exists");
        let mut batch = rusqlite::Batch::new(&self.open_conn.connection, &tables);
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

impl Display for DatabaseTransactionSqlite<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}