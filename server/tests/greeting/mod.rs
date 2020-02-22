use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use server::config::version::ApiVersion;
use server::filters::app_filters;

use crate::common::get_request_endpoint_string;

#[tokio::test]
async fn hello_world() {
    let api = app_filters(ApiVersion::latest());

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
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
    let api = app_filters(ApiVersion::latest());

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello/Joshua").as_ref())
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
