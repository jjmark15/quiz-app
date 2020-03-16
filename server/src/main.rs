#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use server::filters::app_filters;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let api = app_filters();

    const PORT: u16 = 3030;
    info!("welcome to my lovely server");
    warp::serve(api).run(([0, 0, 0, 0], PORT)).await;
}
