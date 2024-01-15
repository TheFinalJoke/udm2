use log;
use postgres::Client;
use crate::db;
use crate::parsers::settings;

pub struct OpenPostgresConnection {
    pub conn: postgres::Client,
}

impl db::EstablishDbConnection for OpenPostgresConnection {
    type UdmConfig = settings::PostgresConfigurer;

    fn establish_connection(settings: settings::PostgresConfigurer) -> Self {
        todo!()
    }
}