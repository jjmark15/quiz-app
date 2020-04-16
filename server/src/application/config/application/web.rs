use crate::application::config::application::env::{EnvReader, Overrideable};
use crate::application::config::application::{ConfigProfile, ProfileDependentConfig};

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct WebConfig<'a, Reader: EnvReader> {
    port: Overrideable<'a, u16, Reader>,
}

impl<Reader: EnvReader> WebConfig<'_, Reader> {
    pub(crate) fn port(&mut self) -> u16 {
        self.port.value()
    }

    pub(crate) fn new(port: Overrideable<'_, u16, Reader>) -> WebConfig<'_, Reader> {
        WebConfig { port }
    }
}

impl<'a, Reader: EnvReader> ProfileDependentConfig<'a, Reader> for WebConfig<'a, Reader> {
    fn with_profile(profile: ConfigProfile, env_reader: &'a Reader) -> Self {
        match profile {
            ConfigProfile::DEFAULT => {
                WebConfig::new(Overrideable::new(3030, "WARPJ_PORT", env_reader))
            }
            ConfigProfile::DEV => WebConfig::new(Overrideable::new(3030, "WARPJ_PORT", env_reader)),
            ConfigProfile::PROD => {
                WebConfig::new(Overrideable::new(8080, "WARPJ_PORT", env_reader))
            }
        }
    }
}
