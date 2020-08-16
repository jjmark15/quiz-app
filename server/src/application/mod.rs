use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use warp::Future;

use quiz_domain::QuizServiceInterface;

use crate::application::config::ApplicationConfig;
use crate::application::error::AppStartupError;
use crate::application::web::routes;

pub(crate) mod config;
mod error;
mod logging;
pub(crate) mod web;

#[derive(Debug, Clone)]
pub struct App {
    socket_address: SocketAddr,
}

impl App {
    pub fn new<'a, QuizService, ConfigReader>(
        config_reader: ConfigReader,
        config_file_path: PathBuf,
    ) -> Result<(Self, impl Future<Output = ()> + 'a), AppStartupError>
    where
        'a: 'static,
        QuizService: 'a + QuizServiceInterface,
        ConfigReader: crate::application::config::ConfigReader<Config = ApplicationConfig>,
    {
        let config: ApplicationConfig = config_reader.with_file_path(config_file_path)?;
        let port: u16 = config.web().port();
        let address = config.web().address();
        Ok(Self::from_ip_and_port::<'a, QuizService>(address, port))
    }

    pub fn from_ip_and_port<'a, QuizService>(
        ip_address: IpAddr,
        port: u16,
    ) -> (Self, impl Future<Output = ()> + 'a)
    where
        'a: 'static,
        QuizService: 'a + QuizServiceInterface,
    {
        let intended_socket_address = SocketAddr::new(ip_address, port);
        let (socket_address, future) = warp::serve(routes::routes::<'a, QuizService>())
            .bind_ephemeral(intended_socket_address);
        (App { socket_address }, future)
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use quiz_domain_mocks::MockQuizService;

    use crate::application::config::ConfigReaderError;
    use crate::MockConfigReader;

    use super::*;

    fn fake_config_path() -> PathBuf {
        PathBuf::from("fake")
    }

    #[test]
    fn fails_to_start_app_if_config_reader_error() {
        let mut config_reader = MockConfigReader::default();
        config_reader
            .expect_with_file_path()
            .with(eq(fake_config_path()))
            .times(1)
            .returning(|_f| Err(ConfigReaderError::BadConfigData));

        let result = App::new::<MockQuizService, MockConfigReader<ApplicationConfig>>(
            config_reader,
            fake_config_path(),
        );

        match result
            .err()
            .expect("Expected app to fail to start but did not")
        {
            AppStartupError::ConfigError(_config_error) => (),
        }
    }
}
