use pkg_version::pkg_version_major;
use warp::test::{request, RequestBuilder};
use warp::Filter;

use quiz_domain::models::quiz::question::QuestionSetImpl;
use quiz_domain::services::quiz::QuizServiceImpl;
use server::routes;

pub(crate) fn get_request_endpoint_string(partial_endpoint: &str) -> String {
    let trimmed_partial = {
        if partial_endpoint.starts_with('/') {
            partial_endpoint.get(1..).unwrap()
        } else {
            partial_endpoint
        }
    };
    vec!["/api".to_string(), trimmed_partial.to_string()].join("/")
}

pub(crate) fn default_application_accept_header() -> String {
    format!("application/vnd.warpj.v{}", pkg_version_major!())
}

pub(crate) fn default_request_builder() -> RequestBuilder {
    request().header("Accept", default_application_accept_header())
}

pub(crate) fn routes_under_test(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    routes::<QuestionSetImpl, QuizServiceImpl>()
}
