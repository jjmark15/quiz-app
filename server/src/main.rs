#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use warp::Filter;

use server::filters::app_filters;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let api = app_filters().recover(server::rejection::handle_api_validation_error);

    const PORT: u16 = 3030;
    info!("welcome to my lovely server");
    warp::serve(api).run(([127, 0, 0, 1], PORT)).await;
}
