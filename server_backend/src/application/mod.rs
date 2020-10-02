use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use warp::Future;

use application_config::ConfigFactory;
pub use application_service::*;

use crate::application::config::ApplicationConfig;
use crate::application::error::AppStartupError;
use crate::application::web::routes;

mod application_service;
pub(crate) mod config;
mod error;
mod logging;
pub(crate) mod web;

#[derive(Debug)]
pub struct App<CfgFactory, AS>
where
    CfgFactory: ConfigFactory<Config = ApplicationConfig>,
    AS: ApplicationService + Send + Sync,
{
    bound_socket_address: Option<SocketAddr>,
    config_factory: CfgFactory,
    application_service: Arc<AS>,
}

impl<CfgFactory, AS> App<CfgFactory, AS>
where
    CfgFactory: ConfigFactory<Config = ApplicationConfig>,
    AS: 'static + ApplicationService + Send + Sync,
{
    pub fn run(
        &mut self,
        config_file_path: PathBuf,
    ) -> Result<impl Future<Output = ()>, AppStartupError>
    where
        CfgFactory: ConfigFactory<Config = ApplicationConfig>,
    {
        let config: ApplicationConfig = self.config_factory.load(config_file_path)?;
        let intended_socket_address: SocketAddr =
            SocketAddr::new(config.web().address(), config.web().port());
        let server = warp::serve(routes::routes(self.application_service.clone()));
        let (bound_socket_address, future) = server.bind_ephemeral(intended_socket_address);
        self.bound_socket_address = Some(bound_socket_address);

        Ok(future)
    }

    pub fn new(config_factory: CfgFactory, application_service: Arc<AS>) -> Self {
        App::<CfgFactory, AS> {
            bound_socket_address: None,
            config_factory,
            application_service,
        }
    }

    pub fn bound_socket_address(&self) -> Option<SocketAddr> {
        self.bound_socket_address
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use application_config::{ApplicationConfigError, ConfigFileReaderError};

    use crate::application::config::application_config_mocks::MockConfigFactory;
    use crate::application::MockApplicationService;

    use super::*;

    #[test]
    fn fails_to_start_app_if_config_reader_error() {
        let config_path = PathBuf::from("fake");
        let mut mock_config_factory = MockConfigFactory::default();
        mock_config_factory
            .expect_load()
            .with(eq(config_path.clone()))
            .returning(|_| {
                Err(ApplicationConfigError::ConfigReaderError(
                    ConfigFileReaderError::BadConfigData,
                ))
            });
        let application_service: Arc<MockApplicationService> =
            Arc::new(MockApplicationService::default());
        let mut app = App::new(mock_config_factory, application_service);

        let result = app.run(config_path);

        match result
            .err()
            .expect("Expected app to fail to start but did not")
        {
            AppStartupError::ConfigError(_config_error) => (),
        }
    }
}
