use core::panic;

use serde::Deserialize;
use crate::parsers::UdmConfig;
use postgres::Config;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UdmConfigurer {
    pub udm: Configurer,
    #[serde(default)]
    pub daemon: DaemonConfigurer,
    #[serde(default)]
    pub command: CommandConfigurer,
}
impl Default for UdmConfigurer {
    fn default() -> Self {
        Self {
            udm: Configurer::default(),
            daemon: DaemonConfigurer::default(),
            command: CommandConfigurer::default(),
        }
    }
}
impl UdmConfig for UdmConfigurer {}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configurer {
    #[serde(default = "default_udm_port")]
    pub port: i64,
}

impl Default for Configurer {
    fn default() -> Self {
        Self {
            port: default_udm_port(),
        }
    }
}
impl UdmConfig for Configurer {}
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
impl UdmConfig for DaemonConfigurer{}

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
impl UdmConfig for SqliteConfigurer{}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostgresConfigurer {
    #[serde(default)]
    user: String, 
    #[serde(default)]
    password: String,
    #[serde(default)]
    db_name: String,
    #[serde(default)]
    db_port: i64,
    #[serde(default)]
    host: String,
    #[serde(default)]
    application_name: Option<String>,
    #[serde(default)]
    options: Option<String>,
}
impl Default for PostgresConfigurer {
    fn default() -> Self {
        Self {
            user: String::from("postgres"),
            password: get_password(),
            db_name: String::from("udm"),
            db_port: 5432,
            host: String::from("localhost"),
            application_name: None,
            options: None,
        }
    }
}
impl Into<Config> for PostgresConfigurer {
    fn into(self) -> Config {
        Config::new()
    }
}
impl UdmConfig for PostgresConfigurer{}
#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandConfigurer {}

impl UdmConfig for CommandConfigurer{}
// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}

fn default_udm_port() -> i64 {
    19211
}

fn get_password() -> String {
    let pass_var = std::env::var_os("UDM_POSTGRES_PASSWORD");
    if let Some(pass) = pass_var {
        return pass.into_string().unwrap()
    } else {
        panic!("Postgres option requires a password")
    }
}