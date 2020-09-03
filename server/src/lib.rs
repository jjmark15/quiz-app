#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use application::config::*;
pub use application::web::routes::routes;
pub use application::App;

mod application;
pub mod cli;
