use quiz_domain::models::quiz::question::{ModelID, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::config::env::EnvReaderImpl;
use crate::application::config::ApplicationConfig;
use crate::application::web::routes;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
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
    pub fn new<'a, QuestionSet, QuizService>() -> (Self, impl Future<Output = ()> + 'a)
    where
        'a: 'static,
        QuestionSet: 'a + QuestionSetInterface<'a>,
        QuestionSet::ID: ModelID<'a>,
        QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
    {
        let mut config = ApplicationConfig::from_env(&EnvReaderImpl);
        let port: u16 = config.web_mut().port();
        Self::from_ip_and_port::<'a, QuestionSet, QuizService>(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
    }

    pub fn from_ip_and_port<'a, QuestionSet, QuizService>(
        ip_address: IpAddr,
        port: u16,
    ) -> (Self, impl Future<Output = ()> + 'a)
    where
        'a: 'static,
        QuestionSet: 'a + QuestionSetInterface<'a>,
        QuestionSet::ID: ModelID<'a>,
        QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
    {
        let intended_socket_address = socket_address_from_ip_and_port(ip_address, port);
        let (socket_address, future) = warp::serve(routes::routes::<'a, QuestionSet, QuizService>())
            .bind_ephemeral(intended_socket_address);
        (App { socket_address }, future)
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }
}

fn socket_address_from_ip_and_port(ip_address: IpAddr, port: u16) -> SocketAddr {
    SocketAddr::new(ip_address, port)
}
