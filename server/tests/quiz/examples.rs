use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;

use crate::common::web::{default_request_builder, routes_under_test, Endpoint};

#[tokio::test]
async fn gives_example_question_set() {
    let api = routes_under_test();

    let resp = default_request_builder()
        .method(Method::GET.as_str())
        .path(Endpoint::ExampleQuizQuestionSet.path_string().as_ref())
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("json response body is an example question set")
        .that(&body)
        .is_equal_to("{\"id\":\"0\",\"name\":\"Example question set title\"}".to_string())
}
