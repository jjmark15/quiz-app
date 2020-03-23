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
        const PORT: u16 = 3030;
        warp::serve(routes::routes())
            .run(([0, 0, 0, 0], PORT))
            .await;
    }
}
