use crate::db;
use crate::db::{DatabaseTransactionsFactory, DbConnection, SqlTableTransactionsFactory};
use crate::parsers::settings;
use async_trait::async_trait;

use tokio_postgres::{Config, NoTls};

pub struct OpenPostgresConnection {
    pub conn: tokio_postgres::Client,
}
#[async_trait]
impl DbConnection for OpenPostgresConnection {}

impl OpenPostgresConnection {
    pub async fn new(settings: settings::PostgresConfigurer) -> Self {
        let config: Config = settings.into();
        let (client, connection) = config
            .connect(NoTls)
            .await
            .unwrap_or_else(|e| panic!("{}", e));
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!(
                    "Unable to establish an connection with postgres database {}",
                    e
                )
            }
        });
        Self { conn: client }
    }
}
#[async_trait]
impl DatabaseTransactionsFactory for OpenPostgresConnection {
    async fn collect_all_current_tables(&mut self) -> crate::UdmResult<Vec<String>> {
        log::debug!("Getting Current tables from protgres database");
        let stmt = self
            .conn
            .prepare("SELECT * FROM pg_catalog.pg_tables")
            .await?;
        let table_rows = self.conn.query(&stmt, &[]).await;
        let mut tables: Vec<String> = Vec::new();
        for row in table_rows? {
            if let Ok(data) = row.try_get(0) {
                tables.push(data);
            }
        }
        log::trace!("Data tables: {:?}", tables);
        Ok(tables)
    }
    async fn gen_schmea(&mut self) -> crate::UdmResult<()> {
        let tables = [
            db::FluidRegulationSchema::create_table(sea_query::PostgresQueryBuilder),
            db::InstructionSchema::create_table(sea_query::PostgresQueryBuilder),
            db::RecipeSchema::create_table(sea_query::PostgresQueryBuilder),
            db::IngredientSchema::create_table(sea_query::PostgresQueryBuilder),
            db::InstructionToRecipeSchema::create_table(sea_query::PostgresQueryBuilder),
        ]
        .join("; ");
        log::debug!("Ensure schmea is defined");
        self.conn.batch_execute(tables.as_str()).await?;
        Ok(())
    }
}
