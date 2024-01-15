use log;
use rusqlite::Connection;
use std::path::Path;
use std::fmt::Display;

use crate::db;
use crate::parsers::settings;

pub struct OpenSqliteConnection {
    pub connection: Connection,
}
impl db::EstablishDbConnection for OpenSqliteConnection {
    type UdmConfig = settings::SqliteConfigurer;

    fn establish_connection(settings: settings::SqliteConfigurer) -> Self {
        let path = Path::new(&settings.db_path);
        log::info!("Using {} as the path for the database", path.display());
        let conn = rusqlite::Connection::open(path)
            .unwrap_or_else(|e| panic!("Error connection to {} due to: {:?}", path.display(), e));
        log::info!("Established Connection with sqlite file");
        OpenSqliteConnection { connection: conn }
    }

}
impl Display for OpenSqliteConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}