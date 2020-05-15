use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;

use crate::common::web::{default_request_builder, routes_under_test, Endpoint};

#[tokio::test]
async fn hello_world() {
    let api = routes_under_test();

    let resp = default_request_builder()
        .method(Method::GET.as_str())
        .path(Endpoint::HelloWorldGreeting.path_string().as_ref())
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("body contains hello world greeting")
        .that(&body)
        .starts_with("Hello, World!")
}

#[tokio::test]
async fn hello_person() {
    let api = routes_under_test();

    let resp = default_request_builder()
        .method(Method::GET.as_str())
        .path(
            Endpoint::HelloNameGreeting("Joshua".to_string())
                .path_string()
                .as_ref(),
        )
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("body contains hello person greeting")
        .that(&body)
        .starts_with("Hello, Joshua!")
}
