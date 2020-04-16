use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;

mod config;
mod error;
mod logging;
mod services;
pub(crate) mod web;

pub struct App;

impl App {
    pub async fn start(&self) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes())
            .run(([0, 0, 0, 0], config.web_mut().port()))
            .await;
    }
}