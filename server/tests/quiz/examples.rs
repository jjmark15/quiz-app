use std::str::from_utf8;

use spectral::prelude::*;
use warp::http::StatusCode;

use crate::common::web::requests::get_example_question_set;
use crate::common::web::routes_under_test;

#[tokio::test]
async fn gives_example_question_set() {
    let api = routes_under_test();

    let resp = get_example_question_set().reply(&api).await;

    asserting("returns OK status code")
        .that(&resp.status())
        .is_equal_to(StatusCode::OK);

    let body: String = from_utf8(resp.body()).unwrap().to_string();

    asserting("json response body is an example question set")
        .that(&body)
        .is_equal_to("{\"id\":\"0\",\"name\":\"Example question set title\"}".to_string())
}
