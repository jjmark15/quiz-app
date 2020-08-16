use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigReaderError {
    #[error("tried to read config file with bad data")]
    BadConfigData,
    #[error("could not read from config file: {0}")]
    MissingConfigFile(std::io::Error),
}
