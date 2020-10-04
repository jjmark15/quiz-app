use std::error::Error;

use warp::Reply;

use crate::ports::http::warp_port::response::{ErrorResponse, ErrorResponseBody, WebErrorResponse};

pub(crate) struct ErrorResponseMapper;

impl ErrorResponseMapper {
    pub(crate) fn map<E: Error + WebErrorResponse>(&self, error: &E) -> ErrorResponse {
        let error_response_body = ErrorResponseBody::new(format!("{}", error));
        ErrorResponse(
            warp::reply::with_status::<warp::reply::Json>(
                error_response_body.into(),
                error.http_status_code(),
            )
            .into_response(),
        )
    }

    pub(crate) fn new() -> Self {
        ErrorResponseMapper
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;

    #[derive(Debug, thiserror::Error)]
    #[error("error message")]
    struct TestWebErrorResponseError;

    impl WebErrorResponse for TestWebErrorResponseError {
        fn http_status_code(&self) -> StatusCode {
            warp::http::StatusCode::BAD_REQUEST
        }
    }

    #[test]
    fn maps_web_error_response_error_to_error_response() {
        let under_test = ErrorResponseMapper::new();
        let _: ErrorResponse = under_test.map(&TestWebErrorResponseError);
    }
}
