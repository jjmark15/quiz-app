use std::path::PathBuf;

use tokio::task::JoinHandle;

use application_config::{
    ConfyConfigFileReader, EnvironmentReaderStd, EnvironmentSupportedConfigTransformerImpl,
    FileReadEnvSupportedConfigFactory, FromEnvironmentSupportedConfig,
};
use quiz_domain::QuizServiceImpl;
use server::{App, ApplicationConfig};

use crate::common::state::web::RequestBuilder;

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
    server_app: App<QuizServiceImpl>,
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

    fn spawn_server_process() -> anyhow::Result<(JoinHandle<()>, App<QuizServiceImpl>)> {
        let (app, future) = server::App::<QuizServiceImpl>::run::<ConfigFactoryAlias>(
            Self::config_factory(),
            Self::config_path(),
        )?;
        Ok((
            tokio::spawn(async move {
                future.await;
            }),
            app,
        ))
    }

    pub(crate) fn server_http_address(&self) -> String {
        format!("http://{}", self.server_app.socket_address().to_string())
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
