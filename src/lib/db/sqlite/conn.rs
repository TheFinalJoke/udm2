use log;
use rusqlite::Connection;
use std::path::Path;
use std::rc::Rc;

use crate::db::NonRelatedTable;
use crate::parsers::settings;

pub struct OpenConnection {
    pub connection: Connection,
    pub settings: Rc<settings::UdmConfigurer>,
}
impl OpenConnection {
    pub fn establish_connection(settings: Rc<settings::UdmConfigurer>) -> OpenConnection {
        let path = Path::new(&settings.daemon.db_path);
        log::info!("Using {} as the path for the database", path.display());
        let conn = rusqlite::Connection::open(path)
            .unwrap_or_else(|e| panic!("Error connection to {} due to: {:?}", path.display(), e));
        OpenConnection {
            connection: conn,
            settings,
        }
    }
}

pub fn create_or_update_database(open_conn: &OpenConnection) -> rusqlite::Result<()> {
    NonRelatedTable::gen_schmea(&open_conn.connection)
}
