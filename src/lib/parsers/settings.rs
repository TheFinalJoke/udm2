use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UdmConfigurer {
    udm: Configurer,
    #[serde(default)]
    daemon: DaemonConfigurer,
    #[serde(default)]
    command: CommandConfigurer,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configurer {
    port: i64,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfigurer {
    #[serde(default="default_daemon_db_path")]
    db_path: String,
}
impl Default for DaemonConfigurer {
    fn default() -> Self {
        Self {
            db_path: default_daemon_db_path(),
        }
    }
}
#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CommandConfigurer {}

impl Default for CommandConfigurer {
    fn default() -> Self {
        Self {

        }
    }
}

// Defaults Funcs

fn default_daemon_db_path() -> String {
    String::from("/etc/udm/udm.db")
}