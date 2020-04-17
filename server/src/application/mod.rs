use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;
use crate::simple_domain_impl::models::quiz::question::{ModelIDImpl, QuestionSetImpl};
use crate::simple_domain_impl::services::quiz::QuizServiceImpl;

mod config;
mod error;
mod logging;
pub(crate) mod web;

pub struct App;

impl App {
    pub async fn start(&self) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes::<
            ModelIDImpl,
            QuestionSetImpl,
            QuizServiceImpl,
        >())
        .run(([0, 0, 0, 0], config.web_mut().port()))
        .await;
    }
}
