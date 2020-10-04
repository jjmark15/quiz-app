use std::error::Error;

use serde::Serialize;
use warp::reply::Json;
use warp::Reply;

pub(crate) struct ErrorResponse(pub(crate) warp::reply::Response);

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}

pub(crate) trait WebErrorResponse: Error {
    fn http_status_code(&self) -> warp::http::StatusCode;

    fn error_response(&self) -> ErrorResponse {
        let error_response_body = ErrorResponseBody::new(format!("{}", self));
        ErrorResponse(
            warp::reply::with_status::<warp::reply::Json>(
                error_response_body.into(),
                self.http_status_code(),
            )
            .into_response(),
        )
    }
}

#[derive(Serialize)]
pub(crate) struct ErrorResponseBody {
    message: String,
}

impl ErrorResponseBody {
    pub(crate) fn new(message: String) -> ErrorResponseBody {
        ErrorResponseBody { message }
    }

    pub(crate) fn message(&self) -> &String {
        &self.message
    }
}

impl Into<warp::reply::Json> for ErrorResponseBody {
    fn into(self) -> Json {
        warp::reply::json(&self)
    }
}
