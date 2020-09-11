use std::net::SocketAddr;
use std::path::PathBuf;

use serde::export::PhantomData;
use warp::Future;

use quiz_domain::QuizServiceInterface;

use crate::application::config::ApplicationConfig;
use crate::application::error::AppStartupError;
use crate::application::web::routes;

pub(crate) mod config;
mod error;
mod logging;
pub(crate) mod web;

#[derive(Debug)]
pub struct App<QuizService: QuizServiceInterface> {
    socket_address: SocketAddr,
    quiz_service: PhantomData<QuizService>,
}

impl<QuizService: 'static + QuizServiceInterface> App<QuizService> {
    pub fn run<ConfigReader>(
        config_reader: ConfigReader,
        config_file_path: PathBuf,
    ) -> Result<(Self, impl Future<Output = ()>), AppStartupError>
    where
        QuizService: QuizServiceInterface,
        ConfigReader: crate::application::config::ConfigReader<Config = ApplicationConfig>,
    {
        let config: ApplicationConfig = config_reader.with_file_path(config_file_path)?;
        let intended_socket_address: SocketAddr =
            SocketAddr::new(config.web().address(), config.web().port());
        let server = warp::serve(routes::routes::<'static, QuizService>());
        let (bound_socket_address, future) = server.bind_ephemeral(intended_socket_address);

        Ok((App::new(bound_socket_address), future))
    }

    fn new(socket_address: SocketAddr) -> Self {
        App::<QuizService> {
            socket_address,
            quiz_service: PhantomData::default(),
        }
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

        let result = App::<MockQuizService>::run::<MockConfigReader<ApplicationConfig>>(
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
