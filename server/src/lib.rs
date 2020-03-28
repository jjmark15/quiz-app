use crate::config::application::env::EnvReaderImpl;
use crate::config::application::ApplicationConfig;

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod filters;
pub(crate) mod handlers;
pub(crate) mod logging;
pub(crate) mod rejection;
pub(crate) mod response;
pub mod routes;

pub struct App;

impl App {
    pub async fn start(&self) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes())
            .run(([0, 0, 0, 0], config.web_mut().port()))
            .await;
    }
}
