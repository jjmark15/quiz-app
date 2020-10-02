use std::env::VarError;

#[cfg(test)]
pub use tests::MockEnvironmentReader;

pub trait EnvironmentReader {
    fn read(&self, key: &str) -> Result<String, EnvironmentReaderError>;
}

#[derive(Debug, Default)]
pub struct EnvironmentReaderStd;

impl EnvironmentReaderStd {
    pub fn new() -> Self {
        EnvironmentReaderStd::default()
    }
}

impl EnvironmentReader for EnvironmentReaderStd {
    fn read(&self, key: &str) -> Result<String, EnvironmentReaderError> {
        Ok(std::env::var(key)?)
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum EnvironmentReaderError {
    #[error("{0}")]
    ReadError(#[from] VarError),
}

#[cfg(test)]
mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub EnvironmentReader {}

        trait EnvironmentReader {
            fn read(&self, key: &str) -> Result<String, EnvironmentReaderError>;
        }
    }
}
