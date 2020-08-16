extern crate pretty_env_logger;

use std::path::PathBuf;

use quiz_domain::QuizServiceImpl;
use server::{App, ApplicationConfig, ConfyConfigReader};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let config_reader: ConfyConfigReader<ApplicationConfig> = ConfyConfigReader::new();
    let mut config_file_path: PathBuf = PathBuf::from(".")
        .join("server")
        .join("configs")
        .join("local");
    config_file_path.set_extension("yml");

    match App::new::<QuizServiceImpl, ConfyConfigReader<ApplicationConfig>>(
        config_reader,
        config_file_path,
    ) {
        Ok((_app, future)) => future.await,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(e.exit_code());
        }
    }
}
