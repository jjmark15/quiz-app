use crate::filters::app_filters;

pub(crate) mod config;
pub(crate) mod error;
pub mod filters;
pub(crate) mod handlers;
pub(crate) mod logging;
pub(crate) mod rejection;
pub(crate) mod response;

pub struct App;

impl App {
    pub async fn start(&self) {
        const PORT: u16 = 3030;
        warp::serve(app_filters()).run(([0, 0, 0, 0], PORT)).await;
    }
}
