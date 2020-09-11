use std::marker::PhantomData;

use crate::application::config::environment::{ConfigEnvironmentError, EnvironmentReader};

pub trait EnvironmentSupportedConfig {
    type Target;

    fn build(
        &self,
        env_reader: &impl EnvironmentReader,
    ) -> Result<Self::Target, ConfigEnvironmentError>;
}

pub trait FromEnvironmentSupportedConfig: Sized {
    type EnvSupportedConfig: EnvironmentSupportedConfig<Target = Self>;

    fn try_from(
        environment_supported_config: Self::EnvSupportedConfig,
        env_reader: &impl EnvironmentReader,
    ) -> Result<Self, ConfigEnvironmentError> {
        environment_supported_config.build(env_reader)
    }
}

pub trait EnvironmentSupportedConfigTransformer {
    type Config: FromEnvironmentSupportedConfig;

    fn transform(
        &self,
        config: <Self::Config as FromEnvironmentSupportedConfig>::EnvSupportedConfig,
    ) -> Result<Self::Config, ConfigEnvironmentError>;
}

#[derive(Debug)]
pub struct EnvironmentSupportedConfigTransformerImpl<
    Cfg: FromEnvironmentSupportedConfig,
    EnvReader: EnvironmentReader,
> {
    config_type_marker: PhantomData<Cfg>,
    env_reader: EnvReader,
}

impl<Cfg: FromEnvironmentSupportedConfig, EnvReader: EnvironmentReader>
    EnvironmentSupportedConfigTransformerImpl<Cfg, EnvReader>
{
    pub fn new(environment_reader: EnvReader) -> Self {
        EnvironmentSupportedConfigTransformerImpl {
            config_type_marker: PhantomData::default(),
            env_reader: environment_reader,
        }
    }
}

impl<Cfg: FromEnvironmentSupportedConfig, EnvReader: EnvironmentReader>
    EnvironmentSupportedConfigTransformer
    for EnvironmentSupportedConfigTransformerImpl<Cfg, EnvReader>
{
    type Config = Cfg;

    fn transform(
        &self,
        config: <Self::Config as FromEnvironmentSupportedConfig>::EnvSupportedConfig,
    ) -> Result<Self::Config, ConfigEnvironmentError> {
        config.build(&self.env_reader)
    }
}

#[cfg(test)]
mockall::mock! {
    pub EnvironmentSupportedConfigTransformer<Cfg: 'static +  FromEnvironmentSupportedConfig> {}

    trait EnvironmentSupportedConfigTransformer {
        type Config = Cfg;

        fn transform(
            &self,
            config: Cfg::EnvSupportedConfig,
        ) -> Result<Cfg, ConfigEnvironmentError>;
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::application::config::environment::MockEnvironmentReader;

    use super::*;

    fn env_transformer<Cfg: FromEnvironmentSupportedConfig>(
        env_reader: MockEnvironmentReader,
    ) -> EnvironmentSupportedConfigTransformerImpl<Cfg, MockEnvironmentReader> {
        EnvironmentSupportedConfigTransformerImpl {
            config_type_marker: PhantomData::default(),
            env_reader,
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct TestConfig {
        field: String,
    }

    impl FromEnvironmentSupportedConfig for TestConfig {
        type EnvSupportedConfig = TestEnvSupportedConfig;
    }

    struct TestEnvSupportedConfig {
        field: String,
    }

    impl EnvironmentSupportedConfig for TestEnvSupportedConfig {
        type Target = TestConfig;

        fn build(
            &self,
            _env_reader: &impl EnvironmentReader,
        ) -> Result<Self::Target, ConfigEnvironmentError> {
            Ok(TestConfig {
                field: self.field.clone(),
            })
        }
    }

    #[test]
    fn transforms_environment_supported_config_into_target_config_type() {
        let mock_env_reader = MockEnvironmentReader::default();
        let transformer: EnvironmentSupportedConfigTransformerImpl<
            TestConfig,
            MockEnvironmentReader,
        > = env_transformer(mock_env_reader);
        let env_supported_config = TestEnvSupportedConfig {
            field: "value".to_string(),
        };
        let expected = TestConfig {
            field: "value".to_string(),
        };

        assert_that(&transformer.transform(env_supported_config).unwrap()).is_equal_to(&expected);
    }
}
