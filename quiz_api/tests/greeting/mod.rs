use crate::common::web::requests::get_request_url;
use spectral::prelude::*;

use crate::common::{state::TestState, web::Endpoint};
use http::StatusCode;

#[tokio::test]
async fn hello_world() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(state.server_http_address(), Endpoint::HelloWorldGreeting);

    state.request_builder().with_url(url).send().await.unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = resp.body().to_string();

    asserting("body contains hello world greeting")
        .that(&body)
        .is_equal_to("Hello, World!".to_string())
}

#[tokio::test]
async fn hello_person() {
    let mut state: TestState = TestState::new();
    let url: String = get_request_url(
        state.server_http_address(),
        Endpoint::HelloNameGreeting("Joshua".to_string()),
    );

    state.request_builder().with_url(url).send().await.unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = resp.body().to_string();

    asserting("body contains hello person greeting")
        .that(&body)
        .is_equal_to("Hello, Joshua!".to_string())
}
