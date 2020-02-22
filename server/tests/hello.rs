use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use server::filters::app_filters;

#[tokio::test]
async fn hello_world() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path("/greeting/hello")
        .reply(&api)
        .await;

    asserting("status code is OK")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("body contains hello world greeting")
        .that(&body)
        .starts_with("Hello, World!")
}

#[tokio::test]
async fn hello_person() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path("/greeting/hello/Joshua")
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
