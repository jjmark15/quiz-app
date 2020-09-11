use crate::application::config::environment::ConfigEnvironmentError;
use crate::ConfigFileReaderError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationConfigError {
    #[error("error occurred reading config file: {0}")]
    ConfigReaderError(#[from] ConfigFileReaderError),
    #[error("error occurred using system environment values to populate application config: {0}")]
    ConfigEnvironmentError(#[from] ConfigEnvironmentError),
}
