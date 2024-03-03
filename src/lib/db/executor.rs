use sea_query::Asterisk;
use sea_query::Iden;
// This will generate all the queries
// This manipluates the data itself
use async_trait::async_trait;
use sea_query::DeleteStatement;
use sea_query::InsertStatement;
use sea_query::Query;
use sea_query::SelectStatement;
use sea_query::UpdateStatement;
use sea_query::SimpleExpr;

#[async_trait]
pub trait GenQueries {
    fn gen_insert_query(&self) -> InsertStatement;
    fn gen_select_query_on_fields<T: Iden + 'static>(
        &self,
        table: T,
        wheres: Vec<SimpleExpr>,
    ) -> SelectStatement {
        let mut binding = Query::select();
        let query = binding.column(Asterisk).from(table);
        for clause in wheres {
            query.and_where(clause);
        }
        query.to_owned()
    }
    fn gen_remove_query(id: i32) -> DeleteStatement;
    fn gen_update_query(&self) -> UpdateStatement;
}
