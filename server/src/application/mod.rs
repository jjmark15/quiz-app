use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;
use std::net::{Ipv4Addr, SocketAddr};
use warp::Future;

mod config;
mod error;
mod logging;
pub(crate) mod web;

#[derive(Debug, Clone)]
pub struct App {
    socket_address: SocketAddr,
}

impl App {
    pub fn new<QuestionSet, QuizService>() -> (Self, impl Future<Output = ()> + 'static)
    where
        QuestionSet: 'static + QuestionSetInterface<'static>,
        QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
    {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        let port: u16 = config.web_mut().port();
        Self::from_port::<QuestionSet, QuizService>(port)
    }

    pub fn from_port<QuestionSet, QuizService>(
        port: u16,
    ) -> (Self, impl Future<Output = ()> + 'static)
    where
        QuestionSet: 'static + QuestionSetInterface<'static>,
        QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
    {
        let intended_socket_address = socket_address_from_port(port);
        let (socket_address, future) = warp::serve(routes::routes::<QuestionSet, QuizService>())
            .bind_ephemeral(intended_socket_address);
        (App { socket_address }, future)
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address.clone()
    }
}

fn socket_address_from_port(port: u16) -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
}
