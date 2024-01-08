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
    #[serde(default = "default_daemon_db_path")]
    pub(crate) db_path: String,
}
impl Default for DaemonConfigurer {
    fn default() -> Self {
        Self {
            db_path: default_daemon_db_path(),
        }
    }
}
#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandConfigurer {}

// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}
