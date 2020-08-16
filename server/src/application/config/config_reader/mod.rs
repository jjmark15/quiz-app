use std::path::PathBuf;

pub use confy_config_reader::*;
pub use error::*;

mod confy_config_reader;
mod error;

pub trait ConfigReader {
    type Config: serde::Serialize + serde::de::DeserializeOwned;

    fn with_file_path(&self, file_path: PathBuf) -> Result<Self::Config, ConfigReaderError>;
}

#[cfg(test)]
mockall::mock! {
    pub ConfigReader<C: 'static + serde::Serialize + serde::de::DeserializeOwned> {}

    trait ConfigReader {
        type Config = C;

        fn with_file_path(
            &self,
            file_path: PathBuf,
        ) -> Result<C, ConfigReaderError>;
    }
}
