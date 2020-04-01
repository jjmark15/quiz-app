use spectral::prelude::*;
use warp::http::{Method, StatusCode};

use crate::common::{default_request_builder, get_request_endpoint_string, routes_under_test};

#[tokio::test]
async fn request_to_invalid_route_returns_not_found_error() {
    let api = routes_under_test();

    let resp = default_request_builder()
        .method(Method::GET.as_str())
        .path(get_request_endpoint_string("/invalid/endpoint").as_ref())
        .reply(&api)
        .await;

    asserting("returns NOT_FOUND status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::NOT_FOUND);
}
