extern crate pretty_env_logger;

use server::App;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let app = App;
    app.start().await;
}
