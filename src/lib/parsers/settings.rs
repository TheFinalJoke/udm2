use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UdmConfigurer {
    udm: Configurer,
    daemon: Option<DaemonConfigurer>,
    command: Option<CommandConfigurer>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configurer {
    port: i64,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfigurer {
    db_path: Option<String>,
}

#[warn(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CommandConfigurer {}
