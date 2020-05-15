use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;

#[cfg(test)]
use mockall::automock;

use crate::application::config::error::Error;
use crate::application::logging::log_string;

#[cfg_attr(test, automock)]
pub(crate) trait EnvReader {
    // Not a static method as there is a limitation with Mockall that makes it harder to mock static methods
    fn read(&self, key: &str) -> Result<String, VarError>;
}

#[derive(Clone, Debug)]
pub(crate) struct EnvReaderImpl;

impl EnvReader for EnvReaderImpl {
    fn read(&self, key: &str) -> Result<String, VarError> {
        env::var(key)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Overrideable<'a, T: FromStr + Eq + PartialEq + Copy, Reader: EnvReader> {
    default: T,
    env_var_key: &'a str,
    value_override: Option<T>,
    tried_override: bool,
    env_reader: &'a Reader,
}

impl<'a, T: FromStr + Eq + PartialEq + Copy, Reader: EnvReader> Overrideable<'a, T, Reader> {
    pub(crate) fn new(
        default_value: T,
        env_var_key: &'a str,
        env_reader: &'a Reader,
    ) -> Overrideable<'a, T, Reader> {
        Overrideable {
            default: default_value,
            env_var_key,
            value_override: Option::None,
            tried_override: false,
            env_reader,
        }
    }

    fn cache_override(&mut self, override_value: T) {
        self.value_override = Some(override_value);
    }

    fn register_tried_override(&mut self) {
        self.tried_override = true;
    }

    fn read_from_env(&mut self) -> T {
        match self.env_reader.read(self.env_var_key) {
            Ok(s) => match T::from_str(s.as_str()) {
                Ok(v) => {
                    self.cache_override(v);
                    v
                }
                Err(_e) => {
                    log::warn!("{}", log_string(&Error::InvalidValueOverride));
                    self.default
                }
            },
            Err(_e) => {
                log::debug!("{}", log_string(&Error::ValueOverrideEnvNotSet));
                self.default
            }
        }
    }

    pub(crate) fn value(&mut self) -> T {
        if !self.tried_override {
            self.register_tried_override();
            self.read_from_env()
        } else if self.value_override.is_some() {
            self.value_override.unwrap()
        } else {
            self.default
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use mockall::predicate::*;
    use spectral::prelude::*;

    use super::*;

    impl Debug for MockEnvReader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MockEnvReader").finish()
        }
    }

    impl PartialEq for MockEnvReader {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    impl Eq for MockEnvReader {}

    pub(crate) fn mock_env_reader(key: &'static str, result_string: &'static str) -> MockEnvReader {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .with(eq(key))
            .returning(move |_| Ok(result_string.to_string()));
        env_reader
    }

    #[test]
    fn overrideable_value_is_overridden_given_override_env_var_is_set() {
        let env_reader = mock_env_reader("TEST_VAR", "1");
        let mut prop = Overrideable::new(0, "TEST_VAR", &env_reader);
        asserting("value is overridden")
            .that(&prop.value())
            .is_equal_to(1)
    }

    #[test]
    fn overrideable_value_is_not_overridden_given_override_env_var_is_not_set() {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .times(1)
            .with(eq("TEST_VAR"))
            .returning(move |_| Err(VarError::NotPresent));
        let mut prop = Overrideable::new(0, "TEST_VAR", &env_reader);
        asserting("value is overridden")
            .that(&prop.value())
            .is_equal_to(0)
    }

    #[test]
    fn only_tries_to_read_from_env_var_once_given_first_is_successful() {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .times(1)
            .with(eq("TEST_VAR"))
            .returning(move |_| Ok("1".to_string()));
        let mut prop = Overrideable::new(0, "TEST_VAR", &env_reader);
        prop.value();
        prop.value();
    }

    #[test]
    fn only_tries_to_read_from_env_var_once_given_first_is_unsuccessful() {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .times(1)
            .with(eq("TEST_VAR"))
            .returning(move |_| Err(VarError::NotPresent));
        let mut prop = Overrideable::new(0, "TEST_VAR", &env_reader);
        prop.value();
        prop.value();
    }

    #[test]
    fn reuses_cached_override_value_if_set() {
        let mut env_reader = MockEnvReader::new();
        env_reader
            .expect_read()
            .times(1)
            .with(eq("TEST_VAR"))
            .returning(move |_| Ok("1".to_string()));
        let mut prop = Overrideable::new(0, "TEST_VAR", &env_reader);
        prop.value();
        asserting("value of 1 is reused")
            .that(&prop.value())
            .is_equal_to(1);
    }
}
