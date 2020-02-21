#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use server::filters::app_filters;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    const PORT: u16 = 3030;
    info!("welcome to my lovely server");
    warp::serve(app_filters()).run(([127, 0, 0, 1], PORT)).await;
}
