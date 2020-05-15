use pkg_version::pkg_version_major;
use warp::test::{request, RequestBuilder};
use warp::Filter;

use quiz_domain::models::quiz::question::QuestionSetImpl;
use quiz_domain::services::quiz::QuizServiceImpl;
use server::routes;

pub(crate) enum Endpoint {
    ApplicationStatus,
    ExampleQuizQuestionSet,
    HelloWorldGreeting,
    HelloNameGreeting(String),
    Invalid,
}

impl Endpoint {
    pub(crate) fn path_string(&self) -> String {
        match self {
            Endpoint::ApplicationStatus => "/admin/status".to_string(),
            Endpoint::ExampleQuizQuestionSet => "/api/quiz/question/set/example".to_string(),
            Endpoint::HelloNameGreeting(name) => format!("/api/greeting/hello/{}", name),
            Endpoint::HelloWorldGreeting => "/api/greeting/hello".to_string(),
            Endpoint::Invalid => "/api/invalid/endpoint".to_string(),
        }
    }
}

pub(crate) fn default_application_accept_header() -> String {
    format!("application/vnd.warpj.v{}", pkg_version_major!())
}

pub(crate) fn routes_under_test(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    routes::<QuestionSetImpl, QuizServiceImpl>()
}

pub(crate) fn default_request_builder() -> RequestBuilder {
    request().header("Accept", default_application_accept_header())
}
