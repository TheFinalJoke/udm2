extern crate log;
use config::{Config, File, FileFormat};

pub mod cli;
pub mod db;
pub mod logger;
pub mod rpc_types;

pub mod udm_traits {
    use std::io::Error;

    pub trait Retrieval<T: 'static> {
        fn retreieve<I: 'static>(self) -> Option<T>;
    }
    pub trait SqlOperations<T> {
        fn add(&self) -> Result<i64, Error>;
        fn modify(&self) -> Result<i64, Error>;
        fn pop(id: i64) -> Result<T, Error>;
        fn drop(id: i64) -> Result<(), Error>;
        fn get(id: i64) -> Result<T, Error>;
    }
}

#[derive(Debug)]
pub struct FileRetrieve {
    pub path: String,
}

impl std::ops::Deref for FileRetrieve {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
impl udm_traits::Retrieval<Config> for FileRetrieve {
    fn retreieve<FileRetrieve>(self) -> Option<Config> {
        log::info!("Using Path {} to build a config", &self.path);
        let file_format = if self.path.as_str().ends_with(".toml") {
            log::info!("Found file to be TOML");
            FileFormat::Toml
        } else {
            log::info!("Found file to be YAML");
            FileFormat::Yaml
        };
        let settings = Config::builder()
            .add_source(File::new(self.path.as_str(), file_format))
            .build();
        log::trace!("Settings ConfigBuild {:?}", &settings);
        Some(settings.unwrap_or_else(|error| panic!("Failed to get Config {}", error)))
    }
}
