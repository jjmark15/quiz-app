use std::path::PathBuf;

use tokio::task::JoinHandle;

use quiz_domain::QuizServiceImpl;
use server::{App, ApplicationConfig, ConfyConfigReader};

use crate::common::state::web::RequestBuilder;

pub(crate) mod web;

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

    fn config_reader() -> ConfyConfigReader<ApplicationConfig> {
        ConfyConfigReader::new()
    }

    fn spawn_server_process() -> anyhow::Result<(JoinHandle<()>, App<QuizServiceImpl>)> {
        let (app, future) = server::App::<QuizServiceImpl>::run::<
            ConfyConfigReader<ApplicationConfig>,
        >(Self::config_reader(), Self::config_path())?;
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
