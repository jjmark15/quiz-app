use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use server::filters::app_filters;

use crate::common::{default_application_accept_header, get_request_endpoint_string};

#[tokio::test]
async fn accept_header_with_valid_api_version_returns_ok() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header(
            "accept",
            format!("{}+text", default_application_accept_header()),
        )
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn accept_header_with_invalid_api_version_returns_not_acceptable() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "application/vnd.warpj.vinvalid+text")
        .reply(&api)
        .await;

    asserting("returns NOT_ACCEPTABLE status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
}

#[tokio::test]
async fn accept_header_with_incorrect_api_version_returns_not_acceptable() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "application/vnd.warpj.v2500+text")
        .reply(&api)
        .await;

    asserting("returns NOT_ACCEPTABLE status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
}

#[tokio::test]
async fn validation_is_skipped_if_accept_header_is_not_present() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn validation_is_skipped_if_client_accepts_any_content_type() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header("accept", "*/*")
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}
