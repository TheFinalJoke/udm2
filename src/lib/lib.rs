use config::Config;
use config::File;
use config::FileFormat;
use std::path::PathBuf;
use std::result;
pub mod db;
pub mod error;
pub mod logger;
pub mod parsers;
pub mod rpc_types;

pub type UdmResult<T> = result::Result<T, error::UdmError>;

pub trait Retrieval<T: 'static> {
    fn retreieve<I: 'static>(self) -> Result<T, String>;
}

#[derive(Debug)]
pub struct FileRetrieve {
    pub path: PathBuf,
}

impl FileRetrieve {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
impl Retrieval<Config> for FileRetrieve {
    fn retreieve<FileRetrieve>(self) -> Result<Config, String> {
        tracing::info!("Using Path {} to build a config", &self.path.display());
        let file_format = if self.path.as_path().extension().unwrap() == "toml" {
            tracing::info!("Found file to be TOML");
            FileFormat::Toml
        } else {
            tracing::info!("Found file to be YAML");
            FileFormat::Yaml
        };
        let settings = Config::builder()
            .add_source(File::new(
                self.path.as_path().to_str().unwrap(),
                file_format,
            ))
            .build();
        tracing::trace!("Settings ConfigBuild {:?}", &settings);
        Ok(settings.unwrap_or_else(|error| panic!("Failed to get Config {}", error)))
    }
}
