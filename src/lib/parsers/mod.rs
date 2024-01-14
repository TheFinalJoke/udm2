use crate::error;
use crate::UdmResult;
use std::rc::Rc;

pub mod settings;

pub fn validate_configurer(configurer: Rc<settings::UdmConfigurer>) -> UdmResult<()> {
    if !configurer.daemon.is_db_set() {
        return Err(error::UdmError::InvalidateConfiguration(String::from(
            "A database is not set, Valid configs are postgres and sqlite",
        )));
    }
    Ok(())
}
