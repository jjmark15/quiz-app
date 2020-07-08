use crate::common::state::web::RequestBuilder;
use quiz_domain::{models::quiz::question::QuestionSetImpl, services::quiz::QuizServiceImpl};
use tokio::task::JoinHandle;

pub(crate) mod web;

#[derive(Debug)]
pub(crate) struct TestState {
    request_builder: RequestBuilder,
    server_proc_handle: JoinHandle<()>,
}

impl TestState {
    pub(crate) fn request_builder(&mut self) -> &mut RequestBuilder {
        &mut self.request_builder
    }

    fn spawn_server_process() -> JoinHandle<()> {
        tokio::spawn(async {
            let app = server::App;
            app.start::<QuestionSetImpl, QuizServiceImpl>().await;
        })
    }
}

impl Default for TestState {
    fn default() -> Self {
        TestState {
            request_builder: RequestBuilder::default(),
            server_proc_handle: Self::spawn_server_process(),
        }
    }
}
