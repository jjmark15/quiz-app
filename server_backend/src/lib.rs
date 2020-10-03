#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use application::config::ApplicationConfig;
pub use application::ApplicationServiceImpl;
pub use server::Server;

mod application;
pub mod cli;
pub(crate) mod ports;
mod quiz_domain;
mod server;
