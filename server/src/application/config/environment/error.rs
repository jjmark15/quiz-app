use crate::application::config::environment::{
    EnvironmentReaderError, EnvironmentVariableConfigValueError,
};

#[derive(Debug, thiserror::Error)]
pub enum ConfigEnvironmentError {
    #[error("{0}")]
    ValueError(#[from] EnvironmentVariableConfigValueError),
    #[error("{0}")]
    ReaderError(#[from] EnvironmentReaderError),
}
