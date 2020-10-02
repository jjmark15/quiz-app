#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use application::config::*;
pub use application::web::routes::routes;
pub use application::ApplicationServiceImpl;
pub use server::Server;

mod application;
pub mod cli;
mod quiz_domain;
mod server;
