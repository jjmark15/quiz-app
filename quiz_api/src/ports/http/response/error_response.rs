use http::StatusCode;
use serde::Serialize;

use crate::ports::logging::{LogEntry, LogEntryKVP};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ErrorResponse {
    body: ErrorResponseBody,
    status_code: StatusCode,
}

impl ErrorResponse {
    pub(crate) fn new(body: ErrorResponseBody, status_code: StatusCode) -> Self {
        ErrorResponse { body, status_code }
    }

    pub(crate) fn body(&self) -> ErrorResponseBody {
        self.body.clone()
    }

    pub(crate) fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

pub(crate) trait WebErrorResponse {
    fn http_status_code(&self) -> StatusCode;
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
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

impl LogEntry for ErrorResponseBody {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "rejection"),
            LogEntryKVP::new("message", self.message()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn error_response_body_returns_a_message() {
        let under_test = ErrorResponseBody::new("message".to_string());
        assert_that(&under_test.message()).is_equal_to(&"message".to_string());
    }

    #[test]
    fn error_response_returns_body() {
        let error_response_body = ErrorResponseBody::new("message".to_string());
        let under_test = ErrorResponse::new(error_response_body.clone(), StatusCode::BAD_REQUEST);
        assert_that(&under_test.body()).is_equal_to(&error_response_body);
    }

    #[test]
    fn error_response_returns_status_code() {
        let error_response_body = ErrorResponseBody::new("message".to_string());
        let under_test = ErrorResponse::new(error_response_body.clone(), StatusCode::BAD_REQUEST);
        assert_that(&under_test.status_code()).is_equal_to(StatusCode::BAD_REQUEST);
    }
}
