#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use application::config::ApplicationConfig;
pub use application::ApplicationServiceImpl;
pub use ports::command_line::CliOptions;
pub use server::Server;

mod application;
pub(crate) mod ports;
mod quiz_domain;
mod server;
