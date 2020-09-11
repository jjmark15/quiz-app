use std::path::PathBuf;

pub use confy_config_file_reader::*;
pub use error::*;

mod confy_config_file_reader;
mod error;

pub trait ConfigFileReader {
    type Config: serde::Serialize + serde::de::DeserializeOwned;

    fn with_file_path(&self, file_path: PathBuf) -> Result<Self::Config, ConfigFileReaderError>;
}

#[cfg(test)]
mockall::mock! {
    pub ConfigFileReader<C: 'static + serde::Serialize + serde::de::DeserializeOwned> {}

    trait ConfigFileReader {
        type Config = C;

        fn with_file_path(
            &self,
            file_path: PathBuf,
        ) -> Result<C, ConfigFileReaderError>;
    }
}
