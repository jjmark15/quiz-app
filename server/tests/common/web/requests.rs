use warp::http::method::Method;
use warp::test::{request, RequestBuilder};

use crate::common::web::{default_application_accept_header, Endpoint};

pub(crate) fn get_application_status() -> RequestBuilder {
    request()
        .method(Method::GET.as_str())
        .path(Endpoint::ApplicationStatus.path_string().as_str())
}

pub(crate) fn get_hello_world_greeting() -> RequestBuilder {
    request()
        .method(Method::GET.as_str())
        .path(Endpoint::HelloWorldGreeting.path_string().as_ref())
}

pub(crate) fn get_hello_name_greeting(name: String) -> RequestBuilder {
    request()
        .method(Method::GET.as_str())
        .path(Endpoint::HelloNameGreeting(name).path_string().as_ref())
}

pub(crate) fn get_example_question_set() -> RequestBuilder {
    default_request_builder()
        .method(Method::GET.as_str())
        .path(Endpoint::ExampleQuizQuestionSet.path_string().as_ref())
}

pub(crate) fn default_request_builder() -> RequestBuilder {
    request().header("Accept", default_application_accept_header())
}
