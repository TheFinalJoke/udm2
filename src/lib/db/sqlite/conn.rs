use crate::db::DbConnection;
use crate::parsers::settings::{self, SqliteConfigurer};
use log;
use rusqlite::Connection;
use std::fmt::Display;
use std::path::Path;

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
impl Display for OpenSqliteConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
