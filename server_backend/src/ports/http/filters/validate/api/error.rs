use thiserror::Error;
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::ports::http::accept_header::ParseAcceptHeaderError;
use crate::ports::http::error::WebErrorResponse;
use crate::ports::http::version::{ApiVersion, ApiVersionImpl};
use crate::ports::logging;
use crate::ports::logging::{LogEntry, LogEntryKVP};

#[derive(Debug, Eq, PartialEq, Error)]
pub(crate) enum ApiValidationError {
    #[error("api version {} is incorrect", .0.version())]
    WrongApiVersion(ApiVersionImpl),
    #[error(transparent)]
    InvalidAcceptHeader(#[from] ParseAcceptHeaderError),
}

impl Reject for ApiValidationError {}

impl WebErrorResponse for ApiValidationError {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
}

impl LogEntry for ApiValidationError {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            logging::LogEntryKVP::new("type", "error"),
            logging::LogEntryKVP::new("message", format!("{}", self)),
        ]
    }
}
