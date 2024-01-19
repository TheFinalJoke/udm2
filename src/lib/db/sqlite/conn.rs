use crate::db::{DbConnection, DatabaseTransactionsFactory, SqlTableTransactionsFactory};
use crate::parsers::settings::{self, SqliteConfigurer};
use log;
use sea_query::SqliteQueryBuilder;
use rusqlite::Connection;
use std::path::Path;
use crate::{db, UdmResult};

pub struct OpenSqliteConnection {
    pub connection: Connection,
    pub settings: SqliteConfigurer,
}

impl DbConnection for OpenSqliteConnection {}

impl OpenSqliteConnection {
    pub fn new(settings: settings::SqliteConfigurer) -> Self {
        let path = Path::new(&settings.db_path);
        log::info!("Using {} as the path for the database", path.display());
        let conn = rusqlite::Connection::open(path)
            .unwrap_or_else(|e| panic!("Error connection to {} due to: {:?}", path.display(), e));
        log::info!("Established Connection with sqlite file");
        OpenSqliteConnection {
            connection: conn,
            settings,
        }
    }
}

impl DatabaseTransactionsFactory for OpenSqliteConnection {
    fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>> {
        log::debug!("Getting current tables in db");
        let mut stmt = self
            .connection
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
        let mut batch = rusqlite::Batch::new(&self.connection, &tables);
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