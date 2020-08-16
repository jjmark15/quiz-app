use crate::ConfigReaderError;

#[derive(Debug, thiserror::Error)]
pub enum AppStartupError {
    #[error("Config error occurred: {0}")]
    ConfigError(#[from] ConfigReaderError),
}

impl AppStartupError {
    pub fn exit_code(&self) -> i32 {
        1
    }
}
