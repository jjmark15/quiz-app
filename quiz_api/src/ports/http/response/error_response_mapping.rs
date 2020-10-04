use std::error::Error;

use crate::ports::http::response::{ErrorResponse, ErrorResponseBody, WebErrorResponse};

pub(crate) struct ErrorResponseMapper;

impl ErrorResponseMapper {
    pub(crate) fn map<E: Error + WebErrorResponse>(&self, error: &E) -> ErrorResponse {
        let error_response_body = ErrorResponseBody::new(format!("{}", error));
        ErrorResponse::new(error_response_body, error.http_status_code())
    }

    pub(crate) fn new() -> Self {
        ErrorResponseMapper
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;
    use spectral::prelude::*;

    #[derive(Debug, thiserror::Error)]
    #[error("error message")]
    struct TestWebErrorResponseError;

    impl WebErrorResponse for TestWebErrorResponseError {
        fn http_status_code(&self) -> StatusCode {
            StatusCode::BAD_REQUEST
        }
    }

    #[test]
    fn maps_web_error_response_error_to_error_response() {
        let under_test = ErrorResponseMapper::new();
        assert_that(&under_test.map(&TestWebErrorResponseError)).is_equal_to(ErrorResponse::new(
            ErrorResponseBody::new("error message".to_string()),
            StatusCode::BAD_REQUEST,
        ))
    }
}
