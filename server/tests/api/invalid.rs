use spectral::prelude::*;

use crate::common::web::requests::get_request_url;
use crate::common::{state::TestState, web::Endpoint};
use http::StatusCode;

#[tokio::test]
async fn request_to_invalid_route_returns_not_found_error() {
    let mut state: TestState = TestState::default();
    let url: String = get_request_url(state.server_http_address(), Endpoint::Invalid);

    state.request_builder().with_url(url).send().await.unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns NOT_FOUND status code")
        .that(resp.status())
        .is_equal_to(StatusCode::NOT_FOUND);
}
