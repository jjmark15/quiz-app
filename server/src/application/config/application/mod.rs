pub use web::*;

use crate::application::config::application::web::WebConfig;
use crate::application::config::environment::{
    ConfigEnvironmentError, EnvironmentReader, EnvironmentSupportedConfig,
};
use crate::FromEnvironmentSupportedConfig;

mod web;

#[derive(Debug, Eq, PartialEq)]
pub struct ApplicationConfig {
    web: WebConfig,
}

impl FromEnvironmentSupportedConfig for ApplicationConfig {
    type EnvSupportedConfig = ApplicationConfigEnvironmentSupported;
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfigEnvironmentSupported {
    web: WebConfigEnvironmentSupported,
}

impl EnvironmentSupportedConfig for ApplicationConfigEnvironmentSupported {
    type Target = ApplicationConfig;

    fn build(
        &self,
        env_reader: &impl EnvironmentReader,
    ) -> Result<Self::Target, ConfigEnvironmentError> {
        Ok(ApplicationConfig {
            web: self.web.build(env_reader)?,
        })
    }
}

impl ApplicationConfig {
    pub fn web(&self) -> &WebConfig {
        &self.web
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig {
            web: WebConfig::default(),
        }
    }
}

impl Default for ApplicationConfigEnvironmentSupported {
    fn default() -> Self {
        ApplicationConfigEnvironmentSupported {
            web: WebConfigEnvironmentSupported::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError;
    use std::net::{IpAddr, Ipv4Addr};

    use predicates::ord::eq;
    use spectral::prelude::*;

    use crate::application::config::environment::{EnvironmentReaderError, MockEnvironmentReader};

    use super::*;

    #[test]
    fn implements_default() {
        ApplicationConfig::default();
    }

    #[test]
    fn environment_supported_implements_default() {
        ApplicationConfig::default();
    }

    #[test]
    fn returns_web_field() {
        let config = ApplicationConfig {
            web: WebConfig::default(),
        };

        asserting("returns web field")
            .that(&config.web())
            .is_equal_to(&WebConfig::default());
    }

    #[test]
    fn environment_supported_config_maps_to_built_config_successfully_when_dependant_configs_map_successfully(
    ) {
        let env_supported = ApplicationConfigEnvironmentSupported {
            web: WebConfigEnvironmentSupported::new(
                "${FAKE_ENV_VAR}".parse().unwrap(),
                IpAddr::from(Ipv4Addr::LOCALHOST),
            ),
        };
        let mut env_reader = MockEnvironmentReader::default();
        env_reader
            .expect_read()
            .with(eq("FAKE_ENV_VAR"))
            .returning(|_| Ok("3030".to_string()));
        let expected = ApplicationConfig {
            web: WebConfig::default(),
        };

        assert_that(&env_supported.build(&env_reader).unwrap()).is_equal_to(&expected);
    }

    #[test]
    fn environment_supported_config_maps_to_built_config_unsuccessfully_when_dependent_configs_map_unsuccessfully(
    ) {
        let env_supported = ApplicationConfigEnvironmentSupported {
            web: WebConfigEnvironmentSupported::new(
                "${FAKE_ENV_VAR}".parse().unwrap(),
                IpAddr::from(Ipv4Addr::LOCALHOST),
            ),
        };
        let mut env_reader = MockEnvironmentReader::default();
        env_reader
            .expect_read()
            .with(eq("FAKE_ENV_VAR"))
            .returning(|_| Err(EnvironmentReaderError::ReadError(VarError::NotPresent)));

        assert_that(&env_supported.build(&env_reader)).is_err();
    }

    #[test]
    fn environment_supported_config_deserializes_successfully_from_yaml_where_fields_are_default_values(
    ) {
        let serialized = "web:\n  port: 3030\n  address: 127.0.0.1";
        let result: serde_yaml::Result<ApplicationConfigEnvironmentSupported> =
            serde_yaml::from_str(serialized);
        let expected = ApplicationConfigEnvironmentSupported {
            web: WebConfigEnvironmentSupported::new(3030.into(), IpAddr::from(Ipv4Addr::LOCALHOST)),
        };

        assert_that(&result.unwrap()).is_equal_to(&expected);
    }
}
