use crate::db::{BinaryType, DatabaseTransactionsFactory, DbConnection};
use crate::parsers::settings::{self, SqliteConfigurer};
use crate::UdmResult;
use async_trait::async_trait;
use std::path::Path;
use tokio_postgres::Row;
use tokio_rusqlite::Connection;

pub struct OpenSqliteConnection {
    pub connection: Connection,
    pub settings: SqliteConfigurer,
}

#[async_trait]
impl DbConnection for OpenSqliteConnection {
    async fn insert(&self, _stmt: String) -> UdmResult<i32> {
        todo!()
    }

    async fn delete(&self, _stmt: String) -> UdmResult<()> {
        todo!()
    }
    async fn update(&self, _stmt: String) -> UdmResult<i32> {
        todo!()
    }
    async fn select(&self, _stmt: String) -> UdmResult<Vec<Row>> {
        todo!()
    }
}

impl OpenSqliteConnection {
    pub async fn new(settings: settings::SqliteConfigurer) -> Self {
        let path = Path::new(&settings.db_path);
        tracing::info!("Using {} as the path for the database", path.display());
        let conn = tokio_rusqlite::Connection::open(path)
            .await
            .unwrap_or_else(|e| panic!("Error connection to {} due to: {:?}", path.display(), e));
        tracing::info!("Established Connection with sqlite file");
        OpenSqliteConnection {
            connection: conn,
            settings,
        }
    }
}

#[async_trait]
impl DatabaseTransactionsFactory for OpenSqliteConnection {
    async fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>> {
        todo!()
        // tracing::debug!("Getting current tables in db");
        // let mut stmt = self.connection.prepare("SELECT name FROM main.sqlite_master WHERE type='table'")?;
        // let table_rows = stmt.query_map([], |rows| rows.get(0))?;
        // let mut tables: Vec<String> = Vec::new();
        // for row in table_rows {
        //     tables.push(row?);
        // }
        // tracing::trace!("Data: tables {:?}", tables);
        // Ok(tables)
    }
    async fn gen_schmea_daemon(&mut self) -> UdmResult<()> {
        todo!()
    }
    async fn gen_schmea_dc(&mut self) -> UdmResult<()> {
        todo!()
        // let tables = [
        //     db::FluidRegulationSchema::create_table(SqliteQueryBuilder),
        //     db::InstructionSchema::create_table(SqliteQueryBuilder),
        //     db::RecipeSchema::create_table(SqliteQueryBuilder),
        //     db::IngredientSchema::create_table(SqliteQueryBuilder),
        //     db::InstructionToRecipeSchema::create_table(SqliteQueryBuilder),
        // ]
        // .join("; ");
        // tracing::debug!("Ensure schmea is defined and exists");
        // let mut batch = rusqlite::Batch::new(&self.connection, &tables);
        // while let Some(mut stmt) = batch.next().unwrap_or_else(|e| {
        //     panic!("Failure creating the schema {:?} with query {:?}", e, batch)
        // }) {
        //     tracing::trace!("Using Batched query {:?}", stmt);
        //     stmt.execute([]).unwrap_or_else(|e| {
        //         panic!(
        //             "Failure creating the schema {:?} with query {:?}",
        //             e,
        //             stmt.expanded_sql()
        //         )
        //     });
        // }
        // tracing::debug!("Tables created");
        // Ok(())
    }
    async fn truncate_schema(&self) -> UdmResult<()> {
        todo!()
    }
    async fn check_and_alter_dbs(&self, _bin_type: BinaryType) -> UdmResult<()> {
        todo!()
    }
}
