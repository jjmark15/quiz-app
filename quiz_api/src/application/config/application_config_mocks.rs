#[cfg(test)]
pub use mocks::*;

#[cfg(test)]
mod mocks {
    use std::path::PathBuf;

    use mockall::mock;

    use application_config::{
        ApplicationConfigError, ConfigFactory, EnvironmentReader, EnvironmentReaderError,
    };

    mock! {
        pub EnvironmentReader {}

        trait EnvironmentReader {
            fn read(&self, key: &str) -> Result<String, EnvironmentReaderError>;
        }
    }

    mock! {
        pub ConfigFactory<Cfg: 'static> {}

        trait ConfigFactory {
            type Config = Cfg;

            fn load(&self, file_path: PathBuf) -> Result<Cfg, ApplicationConfigError>;
        }
    }
}
