use std::marker::PhantomData;
use std::path::PathBuf;

use confy::ConfyError;

use super::{ConfigFileReader, ConfigFileReaderError};

#[derive(Default)]
pub struct ConfyConfigFileReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    config_type: PhantomData<C>,
}

impl<C> ConfigFileReader for ConfyConfigFileReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    type Config = C;

    fn with_file_path(&self, file_path: PathBuf) -> Result<C, ConfigFileReaderError> {
        Ok(confy::load_path(
            file_path,
            confy::MissingConfigFileAction::Nothing,
        )?)
    }
}

impl<C> ConfyConfigFileReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    pub fn new() -> Self {
        ConfyConfigFileReader {
            config_type: Default::default(),
        }
    }
}

impl From<ConfyError> for ConfigFileReaderError {
    fn from(confy_error: ConfyError) -> Self {
        match confy_error {
            ConfyError::GeneralLoadError(e) => ConfigFileReaderError::MissingConfigFile(e),
            ConfyError::BadYamlData(_e) => ConfigFileReaderError::BadConfigData,
            _ => panic!("unexpected confy error occurred {:#?}", confy_error),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use spectral::prelude::*;

    use super::*;

    fn config_path(config_name: String) -> PathBuf {
        let mut path: PathBuf = ["test_data", "config", "config_file_reader", &config_name]
            .iter()
            .collect();
        path.set_extension("yml");
        path
    }

    #[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Debug, Default)]
    struct TestConfig {
        field: String,
    }

    impl TestConfig {
        fn valid() -> Self {
            TestConfig {
                field: "value".to_string(),
            }
        }
    }

    #[test]
    fn reads_valid_config_file_data() {
        let config_file_path = config_path("valid_test_config".to_string());
        let expected = TestConfig::valid();

        let result: Result<TestConfig, ConfigFileReaderError> =
            ConfyConfigFileReader::new().with_file_path(config_file_path);
        asserting("reads valid config file")
            .that(&result.unwrap())
            .is_equal_to(&expected);
    }

    #[test]
    fn fails_to_read_config_file_data_when_missing_required_field() {
        let result: Result<TestConfig, ConfigFileReaderError> = ConfyConfigFileReader::new()
            .with_file_path(config_path("missing_required_field".to_string()));

        match result
            .err()
            .expect("should fail to read invalid config file data")
        {
            ConfigFileReaderError::BadConfigData => (),
            _ => panic!("failed with incorrect config reader error"),
        }
    }

    #[test]
    fn fails_to_read_from_missing_config_file() {
        let result: Result<TestConfig, ConfigFileReaderError> =
            ConfyConfigFileReader::new().with_file_path(config_path("not_there".to_string()));

        match result
            .err()
            .expect("should fail to read from missing config file")
        {
            ConfigFileReaderError::MissingConfigFile(_e) => (),
            _ => panic!("failed with incorrect config reader error"),
        }
    }
}
