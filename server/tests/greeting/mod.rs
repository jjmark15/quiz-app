use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::StatusCode;

use crate::common::web::requests::{get_hello_name_greeting, get_hello_world_greeting};
use crate::common::web::routes_under_test;

#[tokio::test]
async fn hello_world() {
    let api = routes_under_test();

    let resp = get_hello_world_greeting().reply(&api).await;

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
    let api = routes_under_test();

    let resp = get_hello_name_greeting("Joshua".to_string())
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
