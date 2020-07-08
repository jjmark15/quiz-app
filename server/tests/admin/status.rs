use spectral::prelude::*;

use crate::common::web::requests::get_request_url;
use crate::common::{state::TestState, web::Endpoint};
use http::StatusCode;

#[tokio::test]
async fn returns_ok_status_and_body_when_up() {
    let mut state: TestState = TestState::default();
    let url: String = get_request_url("http://localhost:3030", Endpoint::ApplicationStatus);

    state.request_builder().with_url(url).send().await.unwrap();

    let resp = state.request_builder().response().as_ref().unwrap();

    asserting("returns OK status code")
        .that(resp.status())
        .is_equal_to(StatusCode::OK);

    asserting("body contains OK")
        .that(resp.body())
        .is_equal_to("OK".to_string())
}
