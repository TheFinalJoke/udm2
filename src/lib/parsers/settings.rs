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
    #[serde(default = "default_port")]
    pub port: i64,
}

impl Default for Configurer {
    fn default() -> Self {
        Self {
            port: default_port(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfigurer {
    pub postgres: Option<PostgresConfigurer>,
    pub sqlite: Option<SqliteConfigurer>,
}

impl Default for DaemonConfigurer {
    fn default() -> Self {
        Self {
            postgres: Some(PostgresConfigurer::default()),
            sqlite: None,
        }
    }
}

impl DaemonConfigurer {
    pub fn is_db_set(&self) -> bool {
        !self.is_both_db_set() && self.is_a_single_db_set()
    }
    fn is_both_db_set(&self) -> bool {
        self.postgres.is_some() && self.sqlite.is_some()
    }
    fn is_a_single_db_set(&self) -> bool {
        self.postgres.is_some() || self.sqlite.is_some()
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

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostgresConfigurer {}

#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandConfigurer {}

// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}

fn default_port() -> i64 {
    19211
}
