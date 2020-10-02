extern crate pretty_env_logger;

use std::sync::Arc;

use log::info;
use structopt::StructOpt;

use application_config::{
    ConfyConfigFileReader, EnvironmentReaderStd, EnvironmentSupportedConfigTransformerImpl,
    FileReadEnvSupportedConfigFactory, FromEnvironmentSupportedConfig,
};
use quiz_domain::ExampleQuizObjectsServiceImpl;
use server_backend::cli::CliOptions;
use server_backend::{Server, ApplicationConfig, ApplicationServiceImpl};

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
    let mut server = Server::new(config_factory(), application_service());

    match server.run(cli_opts.config_file_path().to_path_buf()) {
        Ok(future) => {
            info!(
                "listening on http://{}",
                server.bound_socket_address().unwrap()
            );
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

fn application_service() -> Arc<ApplicationServiceImpl> {
    Arc::new(ApplicationServiceImpl::new(
        ExampleQuizObjectsServiceImpl::new(),
    ))
}
