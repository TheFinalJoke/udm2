use core::panic;

use crate::parsers::UdmConfig;
use serde::Deserialize;
use tokio_postgres::Config;

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UdmConfigurer {
    pub udm: Configurer,
    #[serde(default)]
    pub daemon: DaemonConfigurer,
    #[serde(default)]
    pub command: CommandConfigurer,
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
impl UdmConfig for DaemonConfigurer {}

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
impl UdmConfig for SqliteConfigurer {}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PostgresConfigurer {
    #[serde(default = "PostgresConfigurer::set_user_default")]
    user: String,
    #[serde(default = "PostgresConfigurer::get_password")]
    password: String,
    #[serde(default = "PostgresConfigurer::set_default_db_name")]
    db_name: String,
    #[serde(default = "PostgresConfigurer::set_default_db_port")]
    db_port: u16,
    #[serde(default = "PostgresConfigurer::set_default_host")]
    host: String,
    #[serde(default)]
    application_name: Option<String>,
    #[serde(default)]
    options: Option<String>,
}
impl Default for PostgresConfigurer {
    fn default() -> Self {
        Self {
            user: Self::set_user_default(),
            password: Self::get_password(),
            db_name: Self::set_default_db_name(),
            db_port: Self::set_default_db_port(),
            host: Self::set_default_host(),
            application_name: None,
            options: None,
        }
    }
}
impl PostgresConfigurer {
    fn set_user_default() -> String {
        String::from("postgres")
    }
    fn get_password() -> String {
        let pass_var = std::env::var_os("UDM_POSTGRES_PASSWORD");
        if let Some(pass) = pass_var {
            pass.into_string().unwrap()
        } else {
            panic!("Postgres option requires a password")
        }
    }
    fn set_default_db_name() -> String {
        String::from("postgres")
    }
    fn set_default_db_port() -> u16 {
        5432
    }
    fn set_default_host() -> String {
        String::from("localhost")
    }
}
#[allow(clippy::from_over_into)]
impl Into<Config> for PostgresConfigurer {
    fn into(self) -> Config {
        Config::new()
            .user(self.user.as_str())
            .password(self.password.as_str())
            .dbname(self.db_name.as_str())
            .port(self.db_port)
            .host(self.host.as_str())
            .application_name(self.application_name.unwrap_or_default().as_str())
            .options(self.options.unwrap_or_default().as_str())
            .to_owned()
    }
}
impl UdmConfig for PostgresConfigurer {}
#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandConfigurer {}

impl UdmConfig for CommandConfigurer {}
// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}

fn default_udm_port() -> i64 {
    19211
}
