extern crate pretty_env_logger;

use structopt::StructOpt;

use quiz_domain::QuizServiceImpl;
use server::cli::CliOptions;
use server::{App, ApplicationConfig, ConfyConfigReader};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let cli_opts: CliOptions = CliOptions::from_args();
    let config_reader: ConfyConfigReader<ApplicationConfig> = ConfyConfigReader::new();

    match App::new::<QuizServiceImpl, ConfyConfigReader<ApplicationConfig>>(
        config_reader,
        cli_opts.config_file_path().to_path_buf(),
    ) {
        Ok((_app, future)) => future.await,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(e.exit_code());
        }
    }
}
