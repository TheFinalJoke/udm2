use log;
use rusqlite::Connection;
use std::path::Path;

use crate::db;
use crate::parsers::settings;

#[derive(Debug)]
pub struct OpenSqliteConnection {
    pub connection: Connection,
}
impl OpenSqliteConnection {
    pub fn establish_connection(settings: &settings::SqliteConfigurer) -> OpenSqliteConnection {
        let binding = settings.clone();
        let path = Path::new(&binding.db_path);
        log::info!("Using {} as the path for the database", path.display());
        let conn = rusqlite::Connection::open(path)
            .unwrap_or_else(|e| panic!("Error connection to {} due to: {:?}", path.display(), e));
        OpenSqliteConnection { connection: conn }
    }
}

impl db::SqlQueryExecutor for OpenSqliteConnection {
    fn execute<T>(&self) -> Result<T, crate::error::UdmError> {
        todo!()
    }
    fn gen_query(&self) -> Box<dyn db::SqlTransactionsFactory> {
        todo!()
    }
}
