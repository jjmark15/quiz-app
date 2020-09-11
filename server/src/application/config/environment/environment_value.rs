use std::str::FromStr;

use serde::export::TryFrom;

use crate::application::config::environment::read_environment::EnvironmentReader;
use crate::application::config::environment::ConfigEnvironmentError;

#[derive(Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "MidDeserializationStepContainer")]
pub(crate) struct EnvironmentReadValue<T: FromStr + Copy> {
    #[serde(skip)]
    environment_variable_key: String,
    initialised_value: Option<T>,
}

impl<T: FromStr + Copy> EnvironmentReadValue<T> {
    pub(crate) fn value<EnvReader: EnvironmentReader>(
        &self,
        environment_reader: &EnvReader,
    ) -> Result<T, ConfigEnvironmentError> {
        match &self.initialised_value {
            Some(value) => Ok(*value),
            None => {
                let value_string: String =
                    environment_reader.read(self.environment_variable_key.as_str())?;
                match T::from_str(value_string.as_str()) {
                    Ok(value) => Ok(value),
                    Err(_e) => Err(EnvironmentVariableConfigValueError::MalformedValue.into()),
                }
            }
        }
    }
}

impl<T: FromStr + Copy> From<T> for EnvironmentReadValue<T> {
    fn from(value: T) -> Self {
        EnvironmentReadValue::<T> {
            environment_variable_key: "".to_string(),
            initialised_value: Some(value),
        }
    }
}

fn env_key_from_env_string(
    env_string: &str,
) -> Result<String, EnvironmentVariableConfigValueError> {
    let prefix_string = "${";
    let suffix_string = '}';
    if !(env_string.starts_with(prefix_string) && env_string.ends_with(suffix_string)) {
        return Err(EnvironmentVariableConfigValueError::MalformedKey(
            env_string.to_string(),
        ));
    }

    let env_key: String = env_string
        .chars()
        .skip(prefix_string.len())
        .take(env_string.len() - (prefix_string.len() + suffix_string.len_utf8()))
        .collect();

    if env_key.is_empty() {
        return Err(EnvironmentVariableConfigValueError::MalformedKey(
            env_string.to_string(),
        ));
    }

    Ok(env_key)
}

impl<T: FromStr + Copy> FromStr for EnvironmentReadValue<T> {
    type Err = EnvironmentVariableConfigValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EnvironmentReadValue::<T> {
            environment_variable_key: env_key_from_env_string(s)?,
            initialised_value: None,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EnvironmentVariableConfigValueError {
    #[error("malformed environment variable string \"{0}\"")]
    MalformedKey(String),
    #[error("could not parse value from environment variable into type")]
    MalformedValue,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
struct MidDeserializationStepContainer(String);

impl<T: FromStr + Copy> TryFrom<MidDeserializationStepContainer> for EnvironmentReadValue<T> {
    type Error = EnvironmentVariableConfigValueError;

    fn try_from(mid_step_container: MidDeserializationStepContainer) -> Result<Self, Self::Error> {
        match mid_step_container.0.parse() {
            Ok(env_key_parsed_value) => Ok(env_key_parsed_value),
            Err(err) => match T::from_str(mid_step_container.0.as_str()) {
                Ok(type_value) => Ok(type_value.into()),
                Err(_) => Err(err),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError;

    use mockall::predicate::eq;
    use spectral::prelude::*;

    use crate::application::config::environment::{
        ConfigEnvironmentError, EnvironmentReaderError, MockEnvironmentReader,
    };

    use super::*;

    #[derive(serde::Deserialize, Eq, PartialEq, Debug)]
    struct DeserializationContainer {
        some_field: EnvironmentReadValue<i8>,
    }

    fn mock_env_reader(
        key: &'static str,
        result: Result<String, EnvironmentReaderError>,
    ) -> MockEnvironmentReader {
        let mut mock_env_reader = MockEnvironmentReader::default();
        mock_env_reader
            .expect_read()
            .with(eq(key))
            .return_const(result);
        mock_env_reader
    }

    #[test]
    fn returns_parsed_value_when_environment_variable_is_set_to_valid_value() {
        let env_var_string = "${MY_ENV_VAR}";
        let mock_env_reader = mock_env_reader("MY_ENV_VAR", Ok("1".to_string()));

        let result: EnvironmentReadValue<i8> =
            EnvironmentReadValue::from_str(env_var_string).unwrap();
        assert_that(&result.value(&mock_env_reader).unwrap()).is_equal_to(1);
    }

    #[test]
    fn returns_error_when_environment_variable_is_set_to_invalid_value() {
        let env_var_string = "${MY_ENV_VAR}";
        let mock_env_reader = mock_env_reader("MY_ENV_VAR", Ok("a".to_string()));

        match EnvironmentReadValue::<i8>::from_str(env_var_string)
            .unwrap()
            .value(&mock_env_reader)
            .err()
            .expect("should fail to parse environment variable value")
        {
            ConfigEnvironmentError::ValueError(
                EnvironmentVariableConfigValueError::MalformedValue,
            ) => (),
            _ => panic!("failed with incorrect error"),
        }
    }

    #[test]
    fn returns_error_when_environment_variable_is_not_set() {
        let env_var_string = "${MY_ENV_VAR}";
        let mock_env_reader = mock_env_reader(
            "MY_ENV_VAR",
            Err(EnvironmentReaderError::ReadError(VarError::NotPresent)),
        );

        match EnvironmentReadValue::<i8>::from_str(env_var_string)
            .unwrap()
            .value(&mock_env_reader)
            .err()
            .expect("should fail to parse environment variable value")
        {
            ConfigEnvironmentError::ReaderError(EnvironmentReaderError::ReadError(
                VarError::NotPresent,
            )) => (),
            _ => panic!("failed with incorrect error"),
        }
    }

    #[test]
    fn parses_successfully_from_environment_variable_when_set_to_valid_string() {
        let env_var_string = "${MY_ENV_VAR}";
        let expected = EnvironmentReadValue::<i8> {
            environment_variable_key: "MY_ENV_VAR".to_string(),
            initialised_value: None,
        };

        let result = EnvironmentReadValue::from_str(env_var_string).unwrap();
        assert_that(&result).is_equal_to(&expected);
    }

    #[test]
    fn parses_unsuccessfully_from_environment_variable_when_set_to_invalid_string() {
        let env_var_string = "${MY_ENV_VAR";

        match EnvironmentReadValue::<i8>::from_str(env_var_string)
            .err()
            .expect("should fail to parse environment variable string")
        {
            EnvironmentVariableConfigValueError::MalformedKey(_s) => (),
            _ => panic!("failed with incorrect error"),
        }
    }

    #[test]
    fn parses_unsuccessfully_from_environment_variable_when_key_set_to_empty_string() {
        let env_var_string = "${}";

        match EnvironmentReadValue::<i8>::from_str(env_var_string)
            .err()
            .expect("should fail to parse environment variable string")
        {
            EnvironmentVariableConfigValueError::MalformedKey(_s) => (),
            _ => panic!("failed with incorrect error"),
        }
    }

    #[test]
    fn returns_initialised_value_when_set() {
        let under_test: EnvironmentReadValue<i8> = 2.into();
        let mock_env_reader = MockEnvironmentReader::default();

        assert_that(&under_test.value(&mock_env_reader).unwrap()).is_equal_to(2);
    }

    #[test]
    fn deserializes_transparently_into_env_var_field_when_set_to_valid_environment_variable_string()
    {
        let serialized = "some_field: ${ENV_KEY}";

        let result: DeserializationContainer = serde_yaml::from_str(serialized)
            .expect("json string should deserialize an instance of the container struct");

        let expected = DeserializationContainer {
            some_field: EnvironmentReadValue::from_str("${ENV_KEY}").unwrap(),
        };

        assert_that(&result).is_equal_to(expected);
    }

    #[test]
    fn deserializes_unsuccessfully_from_environment_variable_when_set_to_invalid_environment_variable_string(
    ) {
        let serialized = "some_field: ${ENV_KEY";

        assert_that(&serde_yaml::from_str::<DeserializationContainer>(
            serialized,
        ))
        .is_err();
    }

    #[test]
    fn deserializes_transparently_into_initialised_value_when_set_to_valid_serialized_type() {
        let serialized = "some_field: 2";

        let result: DeserializationContainer = serde_yaml::from_str(serialized)
            .expect("json string should deserialize an instance of the container struct");

        let expected = DeserializationContainer {
            some_field: EnvironmentReadValue::from(2),
        };

        assert_that(&result).is_equal_to(expected);
    }
}
