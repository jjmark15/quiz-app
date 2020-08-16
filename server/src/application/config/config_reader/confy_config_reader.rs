use std::path::PathBuf;

use serde::export::PhantomData;

use confy::ConfyError;

use crate::{ConfigReader, ConfigReaderError};

#[derive(Default)]
pub struct ConfyConfigReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    config_type: PhantomData<C>,
}

impl<C> ConfigReader for ConfyConfigReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    type Config = C;

    fn with_file_path(&self, file_path: PathBuf) -> Result<C, ConfigReaderError> {
        match confy::load_path(file_path, confy::MissingConfigFileAction::Nothing) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.into()),
        }
    }
}

impl<C> ConfyConfigReader<C>
where
    C: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    pub fn new() -> Self {
        ConfyConfigReader {
            config_type: Default::default(),
        }
    }
}

impl From<ConfyError> for ConfigReaderError {
    fn from(confy_error: ConfyError) -> Self {
        match confy_error {
            confy::ConfyError::GeneralLoadError(e) => ConfigReaderError::MissingConfigFile(e),
            confy::ConfyError::BadYamlData(_e) => ConfigReaderError::BadConfigData,
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
        let mut path = PathBuf::from(".")
            .join("test_data")
            .join("config")
            .join("config_reader")
            .join(config_name);
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

        let result: Result<TestConfig, ConfigReaderError> =
            ConfyConfigReader::new().with_file_path(config_file_path);
        asserting("reads valid config file")
            .that(&result.unwrap())
            .is_equal_to(&expected);
    }

    #[test]
    fn fails_to_read_config_file_data_when_missing_required_field() {
        let result: Result<TestConfig, ConfigReaderError> = ConfyConfigReader::new()
            .with_file_path(config_path("missing_required_field".to_string()));

        match result
            .err()
            .expect("should fail to read invalid config file data")
        {
            ConfigReaderError::BadConfigData => (),
            _ => panic!("failed with incorrect config reader error"),
        }
    }

    #[test]
    fn fails_to_read_from_missing_config_file() {
        let result: Result<TestConfig, ConfigReaderError> =
            ConfyConfigReader::new().with_file_path(config_path("not_there".to_string()));

        match result
            .err()
            .expect("should fail to read from missing config file")
        {
            ConfigReaderError::MissingConfigFile(_e) => (),
            _ => panic!("failed with incorrect config reader error"),
        }
    }
}
