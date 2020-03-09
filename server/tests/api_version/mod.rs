use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use server::filters::app_filters;

use crate::common::{get_request_default_mime_prefix, get_request_endpoint_string};

#[tokio::test]
async fn accept_header_is_valid() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header(
            "accept",
            format!("{}+text", get_request_default_mime_prefix()),
        )
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn accept_header_uses_invalid_api_version() {
    let api = app_filters();

    let resp = request()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/greeting/hello").as_ref())
        .header(
            "accept",
            format!("{}invalid+text", get_request_default_mime_prefix()),
        )
        .reply(&api)
        .await;

    asserting("returns NOT_ACCEPTABLE status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
}
