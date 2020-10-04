use serde::Serialize;
use warp::reply::Json;

pub(crate) struct ErrorResponse(pub(crate) warp::reply::Response);

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}

pub(crate) trait WebErrorResponse {
    fn http_status_code(&self) -> warp::http::StatusCode;
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

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn error_response_body_returns_a_message() {
        let under_test = ErrorResponseBody::new("message".to_string());
        assert_that(&under_test.message()).is_equal_to(&"message".to_string());
    }
}
