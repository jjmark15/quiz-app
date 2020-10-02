use std::net::{IpAddr, Ipv4Addr};

use application_config::{
    ConfigEnvironmentError, EnvironmentReadValue, EnvironmentReader, EnvironmentSupportedConfig,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WebConfig {
    port: u16,
    address: IpAddr,
}

impl WebConfig {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> IpAddr {
        self.address
    }
}

impl Default for WebConfig {
    fn default() -> Self {
        WebConfig {
            port: 3030,
            address: IpAddr::from(Ipv4Addr::LOCALHOST),
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebConfigEnvironmentSupported {
    port: EnvironmentReadValue<u16>,
    address: IpAddr,
}

impl WebConfigEnvironmentSupported {
    #[cfg(test)]
    pub(crate) fn new(port: EnvironmentReadValue<u16>, address: IpAddr) -> Self {
        WebConfigEnvironmentSupported { port, address }
    }
}

impl Default for WebConfigEnvironmentSupported {
    fn default() -> Self {
        WebConfigEnvironmentSupported {
            port: 3030.into(),
            address: IpAddr::from(Ipv4Addr::LOCALHOST),
        }
    }
}

impl EnvironmentSupportedConfig for WebConfigEnvironmentSupported {
    type Target = WebConfig;

    fn build(
        &self,
        env_reader: &impl EnvironmentReader,
    ) -> Result<Self::Target, ConfigEnvironmentError> {
        Ok(WebConfig {
            port: self.port.value(env_reader)?,
            address: self.address,
        })
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn implements_default() {
        WebConfig::default();
    }

    #[test]
    fn environment_supported_implements_default() {
        WebConfigEnvironmentSupported::default();
    }

    fn web_config() -> WebConfig {
        WebConfig {
            port: 3030,
            address: IpAddr::from(Ipv4Addr::LOCALHOST),
        }
    }

    #[test]
    fn returns_port_field() {
        asserting("returns port field")
            .that(&web_config().port())
            .is_equal_to(3030);
    }

    #[test]
    fn returns_address_field() {
        asserting("returns address field")
            .that(&web_config().address())
            .is_equal_to(IpAddr::from(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn environment_supported_config_deserializes_successfully_from_yaml_where_fields_are_default_values(
    ) {
        let serialized = "port: 3030\naddress: 127.0.0.1";
        let result: serde_yaml::Result<WebConfigEnvironmentSupported> =
            serde_yaml::from_str(serialized);
        let expected: WebConfigEnvironmentSupported = WebConfigEnvironmentSupported {
            port: 3030.into(),
            address: IpAddr::from(Ipv4Addr::LOCALHOST),
        };

        assert_that(&result.unwrap()).is_equal_to(&expected);
    }

    #[test]
    fn environment_supported_config_deserializes_successfully_from_yaml_when_port_is_sourced_from_environment_variable(
    ) {
        let serialized = "port: ${FAKE_ENV_VAR}\naddress: 127.0.0.1";
        let result: serde_yaml::Result<WebConfigEnvironmentSupported> =
            serde_yaml::from_str(serialized);
        let expected: WebConfigEnvironmentSupported = WebConfigEnvironmentSupported {
            port: "${FAKE_ENV_VAR}".parse().unwrap(),
            address: IpAddr::from(Ipv4Addr::LOCALHOST),
        };

        assert_that(&result.unwrap()).is_equal_to(&expected);
    }
}
