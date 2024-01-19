use crate::db::DbConnection;
use crate::parsers::{settings, UdmConfig};
use log;
use postgres::Client;
use std::fmt::Display;

pub struct OpenPostgresConnection {
    pub conn: postgres::Client,
}

impl DbConnection for OpenPostgresConnection {}

impl OpenPostgresConnection {
    pub fn new(settings: settings::PostgresConfigurer) -> Self {
        todo!()
    }
}
impl Display for OpenPostgresConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
