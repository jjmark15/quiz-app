#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use server::config::version::ApiVersion;
use server::filters::app_filters;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let api_version: ApiVersion = ApiVersion::latest();
    let api = app_filters(api_version);

    const PORT: u16 = 3030;
    info!("welcome to my lovely server");
    warp::serve(api).run(([127, 0, 0, 1], PORT)).await;
}
