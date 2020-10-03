#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use application::config::*;
pub use application::ApplicationServiceImpl;
pub use ports::http::routes::routes;
pub use server::Server;

mod application;
pub mod cli;
mod ports;
mod quiz_domain;
mod server;
