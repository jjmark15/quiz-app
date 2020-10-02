use std::marker::PhantomData;
use std::path::PathBuf;

use crate::environment::{EnvironmentSupportedConfigTransformer, FromEnvironmentSupportedConfig};
use crate::{ApplicationConfigError, ConfigFileReader};

pub trait ConfigFactory {
    type Config;

    fn load(&self, file_path: PathBuf) -> Result<Self::Config, ApplicationConfigError>;
}

#[derive(Debug)]
pub struct FileReadEnvSupportedConfigFactory<Cfg, FileReader, EnvConfigTransformer>
where
    Cfg: FromEnvironmentSupportedConfig,
    FileReader: ConfigFileReader,
    EnvConfigTransformer: EnvironmentSupportedConfigTransformer<Config = Cfg>,
{
    config_file_reader: FileReader,
    env_config_transformer: EnvConfigTransformer,
    config_type_marker: PhantomData<Cfg>,
}

impl<Cfg, FileReader, EnvConfigTransformer>
    FileReadEnvSupportedConfigFactory<Cfg, FileReader, EnvConfigTransformer>
where
    Cfg: FromEnvironmentSupportedConfig,
    FileReader: ConfigFileReader,
    EnvConfigTransformer: EnvironmentSupportedConfigTransformer<Config = Cfg>,
{
    pub fn new(
        config_file_reader: FileReader,
        env_config_transformer: EnvConfigTransformer,
    ) -> Self {
        FileReadEnvSupportedConfigFactory {
            config_file_reader,
            env_config_transformer,
            config_type_marker: Default::default(),
        }
    }
}

impl<Cfg, FileReader, EnvConfigTransformer> ConfigFactory
    for FileReadEnvSupportedConfigFactory<Cfg, FileReader, EnvConfigTransformer>
where
    Cfg: FromEnvironmentSupportedConfig,
    <Cfg as FromEnvironmentSupportedConfig>::EnvSupportedConfig:
        serde::Serialize + serde::de::DeserializeOwned,
    FileReader: ConfigFileReader<Config = Cfg::EnvSupportedConfig>,
    EnvConfigTransformer: EnvironmentSupportedConfigTransformer<Config = Cfg>,
{
    type Config = Cfg;

    fn load(&self, file_path: PathBuf) -> Result<Self::Config, ApplicationConfigError> {
        let env_supported_config = self.config_file_reader.with_file_path(file_path)?;
        let config = self
            .env_config_transformer
            .transform(env_supported_config)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use predicates::ord::eq;
    use spectral::prelude::*;

    use crate::config_file_reader::{ConfigFileReaderError, MockConfigFileReader};
    use crate::environment::{
        ConfigEnvironmentError, EnvironmentReader, EnvironmentSupportedConfig,
        EnvironmentVariableConfigValueError, MockEnvironmentSupportedConfigTransformer,
    };

    use super::*;

    type TestFileReadEnvSupportedConfigFactory = FileReadEnvSupportedConfigFactory<
        TestConfig,
        MockConfigFileReader<TestConfigEnvironmentSupported>,
        MockEnvironmentSupportedConfigTransformer<TestConfig>,
    >;

    #[derive(Debug, Eq, PartialEq)]
    struct TestConfig {
        plain_field: String,
        env_modified_field: String,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            TestConfig {
                plain_field: "value".to_string(),
                env_modified_field: "modified".to_string(),
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestConfigEnvironmentSupported {
        plain_field: String,
        env_modified_field: String,
    }

    impl Default for TestConfigEnvironmentSupported {
        fn default() -> Self {
            TestConfigEnvironmentSupported {
                plain_field: "value".to_string(),
                env_modified_field: "original".to_string(),
            }
        }
    }

    impl EnvironmentSupportedConfig for TestConfigEnvironmentSupported {
        type Target = TestConfig;

        fn build(
            &self,
            _env_reader: &impl EnvironmentReader,
        ) -> Result<Self::Target, ConfigEnvironmentError> {
            unimplemented!()
        }
    }

    impl FromEnvironmentSupportedConfig for TestConfig {
        type EnvSupportedConfig = TestConfigEnvironmentSupported;
    }

    #[test]
    fn loads_config_successfully_when_file_reader_and_env_transformer_succeed() {
        let expected = TestConfig::default();
        let fake_path = PathBuf::from("fake");

        let mut mock_file_reader = MockConfigFileReader::default();
        mock_file_reader
            .expect_with_file_path()
            .with(eq(fake_path.clone()))
            .returning(|_| Ok(TestConfigEnvironmentSupported::default()));

        let mut mock_env_transformer = MockEnvironmentSupportedConfigTransformer::default();
        mock_env_transformer
            .expect_transform()
            .with(eq(TestConfigEnvironmentSupported::default()))
            .returning(|_| Ok(TestConfig::default()));

        let config_factory: TestFileReadEnvSupportedConfigFactory =
            FileReadEnvSupportedConfigFactory::new(mock_file_reader, mock_env_transformer);

        assert_that(&config_factory.load(fake_path).unwrap()).is_equal_to(&expected);
    }

    #[test]
    fn loads_config_unsuccessfully_when_file_reader_fails() {
        let fake_path = PathBuf::from("fake");

        let mut mock_file_reader = MockConfigFileReader::default();
        mock_file_reader
            .expect_with_file_path()
            .with(eq(fake_path.clone()))
            .returning(|_| Err(ConfigFileReaderError::BadConfigData));

        let mock_env_transformer = MockEnvironmentSupportedConfigTransformer::default();

        let config_factory: TestFileReadEnvSupportedConfigFactory =
            FileReadEnvSupportedConfigFactory::new(mock_file_reader, mock_env_transformer);

        assert_that(&config_factory.load(fake_path)).is_err();
    }

    #[test]
    fn loads_config_unsuccessfully_when_env_transformer_fails() {
        let fake_path = PathBuf::from("fake");

        let mut mock_file_reader = MockConfigFileReader::default();
        mock_file_reader
            .expect_with_file_path()
            .with(eq(fake_path.clone()))
            .returning(|_| Ok(TestConfigEnvironmentSupported::default()));

        let mut mock_env_transformer = MockEnvironmentSupportedConfigTransformer::default();
        mock_env_transformer
            .expect_transform()
            .with(eq(TestConfigEnvironmentSupported::default()))
            .returning(|_| {
                Err(ConfigEnvironmentError::ValueError(
                    EnvironmentVariableConfigValueError::MalformedValue,
                ))
            });

        let config_factory: TestFileReadEnvSupportedConfigFactory =
            FileReadEnvSupportedConfigFactory::new(mock_file_reader, mock_env_transformer);

        assert_that(&config_factory.load(fake_path)).is_err();
    }
}
