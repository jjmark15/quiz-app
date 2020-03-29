extern crate pretty_env_logger;

use server::app::App;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let app = App;
    app.start().await;
}
