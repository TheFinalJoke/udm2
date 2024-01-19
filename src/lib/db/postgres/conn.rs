use crate::db::{DbConnection, DatabaseTransactionsFactory};
use crate::parsers::settings;

use postgres::{Config, NoTls};

pub struct OpenPostgresConnection {
    pub conn: postgres::Client,
}

impl DbConnection for OpenPostgresConnection {}

impl OpenPostgresConnection {
    pub fn new(settings: settings::PostgresConfigurer) -> Self {
        let config: Config = settings.into();
        Self {
            conn: config
                .connect(NoTls)
                .unwrap_or_else(|e| panic!("Unable to connect to postgres database {}", e)),
        }
    }
}
impl DatabaseTransactionsFactory for OpenPostgresConnection {
    fn collect_all_current_tables(&mut self) -> crate::UdmResult<Vec<String>> {
        todo!()
    }
    fn gen_schmea(&mut self) -> crate::UdmResult<()> {
        todo!()
    }
}