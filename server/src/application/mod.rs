use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;

mod config;
mod error;
mod logging;
pub(crate) mod web;

pub struct App;

impl App {
    pub async fn start<
        QuestionSet: 'static + QuestionSetInterface<'static>,
        QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
    >(
        &self,
    ) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes::<QuestionSet, QuizService>())
            .run(([0, 0, 0, 0], config.web_mut().port()))
            .await;
    }
}
