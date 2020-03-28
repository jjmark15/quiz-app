use std::str::FromStr;

use getset::MutGetters;
use log;

use error::Error as ConfigError;

use crate::config::application::env::EnvReader;
use crate::config::application::web::WebConfig;
use crate::logging::log_string;

pub(crate) mod env;
mod error;
mod web;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum ConfigProfile {
    DEFAULT,
    DEV,
    PROD,
}

impl FromStr for ConfigProfile {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();
        if lowercase.eq("default") {
            Ok(ConfigProfile::DEFAULT)
        } else if lowercase.eq("dev") {
            Ok(ConfigProfile::DEV)
        } else if lowercase.eq("prod") {
            Ok(ConfigProfile::PROD)
        } else {
            Err(ConfigError::InvalidProfile)
        }
    }
}

impl ConfigProfile {
    fn from_env<Reader: EnvReader>(env_reader: &Reader) -> Result<ConfigProfile, ConfigError> {
        let key = "WARPJ_ACTIVE_PROFILE";
        match env_reader.read(key) {
            Ok(s) => ConfigProfile::from_str(&s),
            Err(_err) => {
                log::debug!("{}", log_string(&ConfigError::ProfileEnvNotSet));
                Ok(ConfigProfile::DEFAULT)
            }
        }
    }
}

trait ProfileDependentConfig<'a, Reader: EnvReader> {
    fn with_profile(profile: ConfigProfile, env_reader: &'a Reader) -> Self;
}

#[derive(MutGetters, Eq, PartialEq, Debug)]
pub(crate) struct ApplicationConfig<'a, Reader: EnvReader> {
    #[getset(get_mut = "pub")]
    web: WebConfig<'a, Reader>,
}

impl<'a, Reader: EnvReader> ApplicationConfig<'a, Reader> {
    fn with_profile(
        profile: ConfigProfile,
        env_reader: &'a Reader,
    ) -> ApplicationConfig<'a, Reader> {
        ApplicationConfig {
            web: WebConfig::with_profile(profile, env_reader),
        }
    }

    pub(crate) fn from_env(env_reader: &'a Reader) -> ApplicationConfig<'a, Reader> {
        let profile = match ConfigProfile::from_env(env_reader) {
            Ok(p) => p,
            Err(e) => {
                log::warn!("{}", log_string(&e));
                ConfigProfile::DEFAULT
            }
        };
        ApplicationConfig::with_profile(profile, env_reader)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env::VarError;

    use mockall::predicate::*;
    use spectral::prelude::*;

    use env::tests::mock_env_reader;
    use env::MockEnvReader;

    use super::*;

    #[test]
    fn loads_default_application_config_with_no_profile_specified() {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .with(eq("WARPJ_ACTIVE_PROFILE"))
            .returning(move |_| Err(VarError::NotPresent));
        asserting("default application config is loaded")
            .that(&ApplicationConfig::from_env(&env_reader))
            .is_equal_to(&ApplicationConfig {
                web: WebConfig::with_profile(ConfigProfile::DEFAULT, &env_reader),
            });
    }

    #[test]
    fn loads_default_application_config_with_default_profile_specified() {
        let env_reader = mock_env_reader("WARPJ_ACTIVE_PROFILE", "default");
        asserting("default application config is loaded")
            .that(&ApplicationConfig::from_env(&env_reader))
            .is_equal_to(&ApplicationConfig {
                web: WebConfig::with_profile(ConfigProfile::DEFAULT, &env_reader),
            });
    }

    #[test]
    fn loads_dev_application_config_with_dev_profile_specified() {
        let env_reader = mock_env_reader("WARPJ_ACTIVE_PROFILE", "dev");
        asserting("dev application config is loaded")
            .that(&ApplicationConfig::from_env(&env_reader))
            .is_equal_to(&ApplicationConfig {
                web: WebConfig::with_profile(ConfigProfile::DEV, &env_reader),
            });
    }

    #[test]
    fn loads_prod_application_config_with_prod_profile_specified() {
        let env_reader = mock_env_reader("WARPJ_ACTIVE_PROFILE", "prod");
        asserting("prod application config is loaded")
            .that(&ApplicationConfig::from_env(&env_reader))
            .is_equal_to(&ApplicationConfig {
                web: WebConfig::with_profile(ConfigProfile::PROD, &env_reader),
            });
    }
}
