pub use web::*;

use crate::application::config::application::web::WebConfig;

mod web;

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    web: WebConfig,
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

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn implements_default() {
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
}
