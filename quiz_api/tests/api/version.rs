use http::StatusCode;
use serde::Deserialize;
use spectral::prelude::*;

use crate::common::web::requests::get_request_url;
use crate::common::{
    state::TestState,
    web::{default_application_accept_header, Endpoint},
};

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[tokio::test]
async fn accepts_accept_header_with_valid_api_version() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state
        .request_builder()
        .with_url(url)
        .with_header(
            reqwest::header::ACCEPT,
            format!("{}+text", default_application_accept_header()),
        )
        .send()
        .await
        .unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn refuses_accept_header_with_invalid_api_version() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state
        .request_builder()
        .with_url(url)
        .with_header(
            reqwest::header::ACCEPT,
            "application/vnd.warpj.vinvalid+text".to_string(),
        )
        .send()
        .await
        .unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns NOT_ACCEPTABLE status code")
        .that(resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);

    let body: ErrorResponse = resp.json().await.unwrap();

    asserting("body describes api version validation error")
        .that(&body.message)
        .is_equal_to(
            &"bad api version in accept header: cannot parse integer from empty string".to_string(),
        )
}

#[tokio::test]
async fn refuses_accept_header_with_incorrect_api_version() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state
        .request_builder()
        .with_url(url)
        .with_header(
            reqwest::header::ACCEPT,
            "application/vnd.warpj.v2500+text".to_string(),
        )
        .send()
        .await
        .unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns NOT_ACCEPTABLE status code")
        .that(resp.status())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);

    let body: ErrorResponse = resp.json().await.unwrap();

    asserting("body describes api version validation error")
        .that(&body.message)
        .is_equal_to(&"api version 2500 is incorrect".to_string())
}

#[tokio::test]
async fn validation_is_skipped_if_accept_header_is_not_present() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state
        .request_builder()
        .with_url(url)
        .without_header(reqwest::header::ACCEPT)
        .send()
        .await
        .unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);
}

#[tokio::test]
async fn validation_is_skipped_if_client_accepts_any_content_type() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state
        .request_builder()
        .with_url(url)
        .with_header(reqwest::header::ACCEPT, "*/*".to_string())
        .send()
        .await
        .unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);
}
