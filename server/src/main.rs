extern crate pretty_env_logger;

use log::info;
use structopt::StructOpt;

use application_config::{
    ConfyConfigFileReader, EnvironmentReaderStd, EnvironmentSupportedConfigTransformerImpl,
    FileReadEnvSupportedConfigFactory, FromEnvironmentSupportedConfig,
};
use quiz_domain::QuizServiceImpl;
use server::cli::CliOptions;
use server::{App, ApplicationConfig};

type ApplicationConfigEnvSupported =
    <ApplicationConfig as FromEnvironmentSupportedConfig>::EnvSupportedConfig;

type ConfigFactoryAlias = FileReadEnvSupportedConfigFactory<
    ApplicationConfig,
    ConfyConfigFileReader<ApplicationConfigEnvSupported>,
    EnvironmentSupportedConfigTransformerImpl<ApplicationConfig, EnvironmentReaderStd>,
>;

type EnvironmentSupportedConfigTransformerAlias =
    EnvironmentSupportedConfigTransformerImpl<ApplicationConfig, EnvironmentReaderStd>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let cli_opts: CliOptions = CliOptions::from_args();

    match App::<QuizServiceImpl>::run::<ConfigFactoryAlias>(
        config_factory(),
        cli_opts.config_file_path().to_path_buf(),
    ) {
        Ok((app, future)) => {
            info!("listening on http://{}", app.socket_address());
            future.await
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(e.exit_code());
        }
    }
}

fn config_factory() -> ConfigFactoryAlias {
    let config_reader = ConfyConfigFileReader::<ApplicationConfigEnvSupported>::new();
    let env_reader = EnvironmentReaderStd::new();
    let env_config_transformer: EnvironmentSupportedConfigTransformerAlias =
        EnvironmentSupportedConfigTransformerImpl::new(env_reader);
    FileReadEnvSupportedConfigFactory::new(config_reader, env_config_transformer)
}
