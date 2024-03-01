use crate::db::FluidRegulationSchema;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::db::InstructionToRecipeSchema;
use crate::db::RecipeSchema;
use crate::db::{DatabaseTransactionsFactory, DbConnection, SqlTableTransactionsFactory};
use crate::error::UdmError;
use crate::parsers::settings;
use crate::UdmResult;
use async_trait::async_trait;
use log;

use tokio_postgres::{Config, NoTls};

pub struct OpenPostgresConnection {
    pub conn: tokio_postgres::Client,
}
#[async_trait]
impl DbConnection for OpenPostgresConnection {
    async fn insert(&self, stmt: String) -> UdmResult<i32> {
        log::info!("Received insert call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await?;
        let row = self.conn.query_one(&prepared, &[]).await?;
        let data: UdmResult<i32> = row
            .try_get(0)
            .map_err(|e| UdmError::ApiFailure(e.to_string()));
        log::debug!("Result from inserting into db {:?}", &data);
        data
    }
}

impl OpenPostgresConnection {
    pub async fn new(settings: settings::PostgresConfigurer) -> Self {
        let config: Config = settings.into();
        let (client, connection) = config.connect(NoTls).await.unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(15)
        });
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!(
                    "Unable to establish an connection with postgres database {}",
                    e
                );
                std::process::exit(10)
            }
        });
        Self { conn: client }
    }
    pub async fn collect_current_dbs(&mut self) -> UdmResult<Vec<String>> {
        log::debug!("Collecting Current databases");
        let sql = "SELECT datname FROM pg_database";
        let stmt = self.conn.prepare(sql).await?;
        let collected_db_rows = self.conn.query(&stmt, &[]).await;
        Self::from_row_to_vec_string(collected_db_rows?)
    }
    fn from_row_to_vec_string(rows: Vec<tokio_postgres::Row>) -> UdmResult<Vec<String>> {
        let mut tables: Vec<String> = Vec::new();
        for row in rows {
            if let Ok(data) = row.try_get(0) {
                tables.push(data);
            }
        }
        log::trace!("Data tables: {:?}", tables);
        Ok(tables)
    }
}
#[async_trait]
impl DatabaseTransactionsFactory for OpenPostgresConnection {
    async fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>> {
        log::debug!("Getting Current tables from protgres database");
        let stmt = self
            .conn
            .prepare("SELECT * FROM pg_catalog.pg_tables")
            .await?;
        let table_rows = self.conn.query(&stmt, &[]).await;
        Self::from_row_to_vec_string(table_rows?)
    }
    async fn gen_schmea(&mut self) -> UdmResult<()> {
        let tables = [
            FluidRegulationSchema::create_table(sea_query::PostgresQueryBuilder),
            InstructionSchema::create_table(sea_query::PostgresQueryBuilder),
            RecipeSchema::create_table(sea_query::PostgresQueryBuilder),
            IngredientSchema::create_table(sea_query::PostgresQueryBuilder),
            InstructionToRecipeSchema::create_table(sea_query::PostgresQueryBuilder),
        ]
        .join("; ");
        log::debug!("Ensure schmea is defined");
        if let Err(query_err) = self.conn.batch_execute(tables.as_str()).await {
            log::error!("{}", query_err);
            std::process::exit(20)
        }
        Ok(())
    }
    async fn truncate_schema(&self) -> UdmResult<()> {
        let tables =
            r#""InstructionToRecipe", "Ingredient", "Recipe", "Instruction", "FluidRegulation""#;
        let query = format!("TRUNCATE TABLE {};", tables);
        log::info!("Running query: {}", &query);
        self.conn
            .batch_execute(query.as_str())
            .await
            .map_err(|e| UdmError::ApiFailure(e.to_string()))
    }
}
