use crate::db::BinaryType;
use crate::db::DatabaseTransactionsFactory;
use crate::db::DbConnection;
use crate::db::FluidRegulationSchema;
use crate::db::IngredientSchema;
use crate::db::InstructionSchema;
use crate::db::InstructionToRecipeSchema;
use crate::db::PumpLogSchema;
use crate::db::RecipeSchema;
use crate::db::SqlTableTransactionsFactory;
use crate::error::trace_log_error;
use crate::error::UdmError;
use crate::parsers::settings;
use crate::UdmResult;
use async_trait::async_trait;
use tokio_postgres::Config;
use tokio_postgres::NoTls;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct OpenPostgresConnection {
    pub conn: tokio_postgres::Client,
}
#[async_trait]
impl DbConnection for OpenPostgresConnection {
    async fn insert(&self, stmt: String) -> UdmResult<i32> {
        tracing::info!("Received insert call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        let row = self.conn.query_one(&prepared, &[]).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        let data: UdmResult<i32> = row
            .try_get(0)
            .map_err(|e| trace_log_error(UdmError::ApiFailure(e.to_string())));
        tracing::debug!("Result from inserting into db {:?}", &data);
        data
    }
    async fn insert_with_uuid(&self, stmt: String) -> UdmResult<Uuid> {
        tracing::info!("Received insert call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        let row = self.conn.query_one(&prepared, &[]).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        let data: UdmResult<Uuid> = row
            .try_get(0)
            .map_err(|e| trace_log_error(UdmError::ApiFailure(e.to_string())));
        tracing::debug!("Result from inserting into db {:?}", &data);
        data
    }
    async fn delete(&self, stmt: String) -> UdmResult<()> {
        tracing::info!("Received delete call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await?;
        let result = self.conn.query_opt(&prepared, &[]).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        tracing::debug!("Result from deleting from db: {:?}", &result);
        Ok(())
    }
    async fn update(&self, stmt: String) -> UdmResult<i32> {
        tracing::info!("Received update call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await?;
        let row = self.conn.query_one(&prepared, &[]).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        let data: UdmResult<i32> = row
            .try_get(0)
            .map_err(|e| trace_log_error(UdmError::ApiFailure(e.to_string())));
        tracing::debug!("Result from inserting into db {:?}", &data);
        data
    }
    async fn select(&self, stmt: String) -> UdmResult<Vec<Row>> {
        tracing::info!("Received update call query: {}", &stmt);
        let prepared = self.conn.prepare(stmt.as_str()).await?;
        let rows = self.conn.query(&prepared, &[]).await.map_err(|e| {
            tracing::error!("{}", e.to_string());
            trace_log_error(UdmError::ApiFailure(e.to_string()))
        })?;
        tracing::debug!("Result from inserting into db {:?}", &rows);
        Ok(rows)
    }
}

impl OpenPostgresConnection {
    pub async fn new(settings: settings::PostgresConfigurer) -> Self {
        let config: Config = settings.into();
        let (client, connection) = config.connect(NoTls).await.unwrap_or_else(|e| {
            tracing::error!("{}", e);
            std::process::exit(15)
        });
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                tracing::error!(
                    "Unable to establish an connection with postgres database {}",
                    e
                );
                std::process::exit(10)
            }
        });
        Self { conn: client }
    }
    pub async fn collect_current_dbs(&mut self) -> UdmResult<Vec<String>> {
        tracing::debug!("Collecting Current databases");
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
        tracing::trace!("Data tables: {:?}", tables);
        Ok(tables)
    }
}
#[async_trait]
impl DatabaseTransactionsFactory for OpenPostgresConnection {
    async fn collect_all_current_tables(&mut self) -> UdmResult<Vec<String>> {
        tracing::debug!("Getting Current tables from protgres database");
        let stmt = self
            .conn
            .prepare("SELECT * FROM pg_catalog.pg_tables")
            .await?;
        let table_rows = self.conn.query(&stmt, &[]).await;
        Self::from_row_to_vec_string(table_rows?)
    }
    async fn gen_schmea_daemon(&mut self) -> UdmResult<()> {
        let tables = [
            FluidRegulationSchema::create_table(sea_query::PostgresQueryBuilder),
            InstructionSchema::create_table(sea_query::PostgresQueryBuilder),
            RecipeSchema::create_table(sea_query::PostgresQueryBuilder),
            IngredientSchema::create_table(sea_query::PostgresQueryBuilder),
            InstructionToRecipeSchema::create_table(sea_query::PostgresQueryBuilder),
        ]
        .join("; ");
        tracing::debug!("Ensure schmea is defined");
        if let Err(query_err) = self.conn.batch_execute(tables.as_str()).await {
            tracing::error!("{}", query_err);
            std::process::exit(20)
        }
        Ok(())
    }
    async fn gen_schmea_dc(&mut self) -> UdmResult<()> {
        let tables = [PumpLogSchema::create_table(sea_query::PostgresQueryBuilder)].join("; ");
        tracing::debug!("Ensure schmea is defined");
        if let Err(query_err) = self.conn.batch_execute(tables.as_str()).await {
            tracing::error!("{}", query_err);
            std::process::exit(20)
        }
        Ok(())
    }
    async fn truncate_schema(&self) -> UdmResult<()> {
        let tables = r#""InstructionToRecipe", "Ingredient", "Recipe", "Instruction", "FluidRegulation", "Pumplog""#;
        let query = format!("TRUNCATE TABLE {};", tables);
        tracing::info!("Running query: {}", &query);
        self.conn
            .batch_execute(query.as_str())
            .await
            .map_err(|e| trace_log_error(UdmError::ApiFailure(e.to_string())))
    }
    async fn check_and_alter_dbs(&self, _bin_type: BinaryType) -> UdmResult<()> {
        todo!()
        // I hope to finish this, but will take some serious redesign
        // let tables = match bin_type {
        //     BinaryType::Daemon => HashMap::from([
        //         (
        //             FluidRegulationSchema::Table.column_to_str(),
        //             [
        //                 FluidRegulationSchema::FrId.column_to_str(),
        //                 FluidRegulationSchema::GpioPin.column_to_str(),
        //                 FluidRegulationSchema::RegulatorType.column_to_str(),
        //                 FluidRegulationSchema::PumpNum.column_to_str(),
        //             ]
        //             .to_vec(),
        //         ),
        //         (
        //             InstructionToRecipeSchema::Table.column_to_str(),
        //             [
        //                 InstructionToRecipeSchema::Id.column_to_str(),
        //                 InstructionToRecipeSchema::InstructionId.column_to_str(),
        //                 InstructionToRecipeSchema::InstructionOrder.column_to_str(),
        //                 InstructionToRecipeSchema::RecipeId.column_to_str(),
        //             ]
        //             .to_vec(),
        //         ),
        //         (
        //             IngredientSchema::Table.column_to_str(),
        //             [
        //                 IngredientSchema::IngredientId.column_to_str(),
        //                 IngredientSchema::Name.column_to_str(),
        //                 IngredientSchema::Alcoholic.column_to_str(),
        //                 IngredientSchema::Description.column_to_str(),
        //                 IngredientSchema::IsActive.column_to_str(),
        //                 IngredientSchema::FrId.column_to_str(),
        //                 IngredientSchema::Amount.column_to_str(),
        //                 IngredientSchema::IngredientType.column_to_str(),
        //                 IngredientSchema::InstructionId.column_to_str(),
        //             ]
        //             .to_vec(),
        //         ),
        //         (
        //             InstructionSchema::Table.column_to_str(),
        //             [
        //                 InstructionSchema::InstructionId.column_to_str(),
        //                 InstructionSchema::InstructionDetail.column_to_str(),
        //                 InstructionSchema::InstructionName.column_to_str(),
        //             ]
        //             .to_vec(),
        //         ),
        //         (
        //             RecipeSchema::Table.column_to_str(),
        //             [
        //                 RecipeSchema::RecipeId.column_to_str(),
        //                 RecipeSchema::Name.column_to_str(),
        //                 RecipeSchema::UserInput.column_to_str(),
        //                 RecipeSchema::DrinkSize.column_to_str(),
        //                 RecipeSchema::Description.column_to_str(),
        //             ]
        //             .to_vec(),
        //         ),
        //     ]),
        //     BinaryType::DrinkCtrl => HashMap::from([(
        //         PumpLogSchema::Table.column_to_str(),
        //         [
        //             PumpLogSchema::ReqId.column_to_str(),
        //             PumpLogSchema::ReqType.column_to_str(),
        //             PumpLogSchema::FluidId.column_to_str(),
        //         ]
        //         .to_vec(),
        //     )]),
        //     BinaryType::Bin => {
        //         unimplemented!()
        //     }
        // };
        // for (table, columns) in tables.into_iter() {
        //     let results = self
        //         .conn
        //         .query(BASE_COLLECT_FIELDS, &[&table])
        //         .await
        //         .map_err(|e| trace_log_error(UdmError::ApiFailure(e.to_string()))?;
        //     for row in results {
        //         let column = &row.try_get(0)?;
        //         if !columns.contains(column) {
        //             match table {
        //                 "InstructionToRecipe" => InstructionToRecipeSchema::alter_table(
        //                     PostgresQueryBuilder,
        //                     InstructionToRecipeSchema::from_str(column).unwrap(),
        //                 ),
        //                 "Ingredient" => Schema,
        //                 "Recipe" => Schema,
        //                 "Instruction" => Schema,
        //                 "FluidRegulation" => Schema,
        //                 "Pumplog" => Schema,
        //             }
        //         }
        //     }
        // const BASE_COLLECT_FIELDS: &str =
        //    "SELECT * FROM information_schema.columns WHERE table_schema = 'public' and table_name = $1";
        // }
        // Ok(())
    }
}
