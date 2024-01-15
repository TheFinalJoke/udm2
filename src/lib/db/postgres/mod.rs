use postgres::{self, Client};
use crate::db;

pub mod conn;

pub struct NonRelatedTablePostgres {
    open_conn: Client,
}

impl db::DatabaseTransactionsFactory for NonRelatedTablePostgres {
    fn collect_all_current_tables(&mut self) -> crate::UdmResult<Vec<String>> {
        log::debug!("Geing current tables in db via postgres");
        let query = "SELECT tablename FROM information_schema.tables WHERE table_schema = 'public'";
        log::trace!("Using query {}", &query);
        let stmt = self.open_conn.prepare(query)?;
        let rows = self.open_conn.query(&stmt, &[])?;
        let mut table_rows = Vec::new();
        for row in rows {
            table_rows.push(row.get(0))
        }
        Ok(table_rows)
    }
    fn gen_schmea(&mut self) -> crate::UdmResult<()> {
        todo!()
    }
}