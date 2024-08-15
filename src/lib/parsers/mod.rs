use crate::error;
use crate::UdmResult;
use serde::Deserialize;
use std::fmt::Debug;
use std::sync::Arc;

pub mod settings;

pub trait UdmConfig: for<'a> Deserialize<'a> + Debug + Default {}

pub fn validate_configurer(configurer: Arc<settings::UdmConfigurer>) -> UdmResult<()> {
    if !configurer.is_db_set() {
        return Err(error::UdmError::InvalidateConfiguration(String::from(
            "A database is not set, Valid configs are postgres and sqlite",
        )));
    }
    tracing::info!("Configuration has been validated. NO ERRORS!");
    Ok(())
}
