use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UdmConfigurer {
    pub udm: Configurer,
    #[serde(default)]
    pub daemon: DaemonConfigurer,
    #[serde(default)]
    pub command: CommandConfigurer,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configurer {
    pub port: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfigurer {
    pub postgres: Option<PostgresConfigurer>,
    pub sqlite: Option<SqliteConfigurer>
}
impl Default for DaemonConfigurer {
    fn default() -> Self {
        Self {
            postgres: Some(PostgresConfigurer::default()),
            sqlite: None
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SqliteConfigurer {
    #[serde(default = "default_daemon_db_path")]
    pub(crate) db_path: String,
}
impl Default for SqliteConfigurer {
    fn default() -> Self {
        Self {
            db_path: default_daemon_db_path(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostgresConfigurer {

}
impl Default for PostgresConfigurer {
    fn default() -> Self {
        Self {}
    }
}
#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandConfigurer {}

// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}
