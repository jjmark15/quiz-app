use crate::config::application::env::EnvReaderImpl;
use crate::config::application::ApplicationConfig;
use crate::web::routes;

pub struct App;

impl App {
    pub async fn start(&self) {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        warp::serve(routes::routes())
            .run(([0, 0, 0, 0], config.web_mut().port()))
            .await;
    }
}
