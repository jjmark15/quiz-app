use quiz_domain::models::quiz::question::QuestionSetImpl;
use quiz_domain::services::quiz::QuizServiceImpl;

use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;

mod config;
mod error;
mod logging;
pub(crate) mod web;

pub struct App;

impl App {
    pub async fn start(&self) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes::<QuestionSetImpl, QuizServiceImpl>())
            .run(([0, 0, 0, 0], config.web_mut().port()))
            .await;
    }
}
