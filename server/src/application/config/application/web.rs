use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn implements_default() {
        WebConfig::default();
    }

    fn web_config() -> WebConfig {
        WebConfig::default()
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
}
