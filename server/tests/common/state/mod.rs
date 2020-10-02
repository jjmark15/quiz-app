use std::path::PathBuf;

use tokio::task::JoinHandle;

use application_config::{
    ConfyConfigFileReader, EnvironmentReaderStd, EnvironmentSupportedConfigTransformerImpl,
    FileReadEnvSupportedConfigFactory, FromEnvironmentSupportedConfig,
};
use server::{App, ApplicationConfig, ApplicationServiceImpl};

use crate::common::state::web::RequestBuilder;
use quiz_domain::ExampleQuizObjectsServiceImpl;
use std::sync::Arc;

pub(crate) mod web;

type ApplicationConfigEnvSupported =
    <ApplicationConfig as FromEnvironmentSupportedConfig>::EnvSupportedConfig;

type ConfigFactoryAlias = FileReadEnvSupportedConfigFactory<
    ApplicationConfig,
    ConfyConfigFileReader<ApplicationConfigEnvSupported>,
    EnvironmentSupportedConfigTransformerImpl<ApplicationConfig, EnvironmentReaderStd>,
>;

type EnvironmentSupportedConfigTransformerAlias =
    EnvironmentSupportedConfigTransformerImpl<ApplicationConfig, EnvironmentReaderStd>;

#[derive(Debug)]
pub(crate) struct TestState {
    request_builder: RequestBuilder,
    server_proc_handle: JoinHandle<()>,
    server_app: App<ConfigFactoryAlias, ApplicationServiceImpl>,
}

impl TestState {
    pub(crate) fn request_builder(&mut self) -> &mut RequestBuilder {
        &mut self.request_builder
    }

    fn config_path() -> PathBuf {
        ["configs", "integration.yml"].iter().collect()
    }

    fn config_factory() -> ConfigFactoryAlias {
        let config_reader = ConfyConfigFileReader::<ApplicationConfigEnvSupported>::new();
        let env_reader = EnvironmentReaderStd::new();
        let env_config_transformer: EnvironmentSupportedConfigTransformerAlias =
            EnvironmentSupportedConfigTransformerImpl::new(env_reader);
        FileReadEnvSupportedConfigFactory::new(config_reader, env_config_transformer)
    }

    fn application_service() -> Arc<ApplicationServiceImpl> {
        Arc::new(ApplicationServiceImpl::new(
            ExampleQuizObjectsServiceImpl::new(),
        ))
    }

    fn spawn_server_process() -> anyhow::Result<(
        JoinHandle<()>,
        App<ConfigFactoryAlias, ApplicationServiceImpl>,
    )> {
        let mut server_app = server::App::new(Self::config_factory(), Self::application_service());
        let future = server_app.run(Self::config_path())?;
        Ok((
            tokio::spawn(async move {
                future.await;
            }),
            server_app,
        ))
    }

    pub(crate) fn server_http_address(&self) -> String {
        format!(
            "http://{}",
            self.server_app.bound_socket_address().unwrap().to_string()
        )
    }

    pub(crate) fn new() -> Self {
        let (join_handle, server_app) =
            Self::spawn_server_process().expect("failed to spawn server process");
        TestState {
            request_builder: RequestBuilder::default(),
            server_proc_handle: join_handle,
            server_app,
        }
    }
}
