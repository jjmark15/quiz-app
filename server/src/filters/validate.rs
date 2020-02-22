use std::convert::Infallible;

use serde::Serialize;
use warp::reply::Response;
use warp::{reject, Filter, Rejection, Reply};

use crate::config::version::ApiVersion;

pub fn api_version() -> impl Filter<Extract = (ApiVersion,), Error = Rejection> + Copy {
    warp::header::<String>("Accept")
        .and_then(|_h: String| async move { Err(reject::custom(InvalidApiVersionHeader)) })
        .recover(handle_rejection)
}

impl warp::reply::Reply for ApiVersion {
    fn into_response(self) -> Response {
        unimplemented!()
    }
}

#[derive(Debug)]
struct InvalidApiVersionHeader;

impl reject::Reject for InvalidApiVersionHeader {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&ErrorMessage {
        code: 200,
        message: "hi".to_string(),
    });

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;
    use warp::test::request;

    use super::*;

    #[tokio::test]
    async fn rejects_invalid_version_tag() {
        let api = api_version();

        let resp = request()
            .header("Accept", "text/vnd.warpj.missing+plain")
            .reply(&api)
            .await;

        asserting("rejects with status code 400")
            .that(&resp.status().as_u16())
            .is_equal_to(400)
    }
}
