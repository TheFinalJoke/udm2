// This will generate all the queries
// This manipluates the data itself
use crate::UdmResult;
use sea_query::InsertStatement;
use async_trait::async_trait;

#[async_trait]
pub trait SqlQueryExecutor{
    async fn insert_into_db(&self) -> UdmResult<()>;
    fn gen_insert_query(&self) -> InsertStatement;
}
