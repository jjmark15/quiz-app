use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::method::Method;
use warp::http::StatusCode;
use warp::test::request;

use crate::common::web::{routes_under_test, Endpoint};

#[tokio::test]
async fn returns_ok_status_and_body_when_up() {
    let api = routes_under_test();

    let resp = request()
        .method(Method::GET.as_str())
        .path(Endpoint::ApplicationStatus.path_string().as_str())
        .reply(&api)
        .await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    asserting("body contains OK")
        .that(&from_utf8(resp.body()).unwrap().to_string().eq("OK"))
        .is_true()
}
