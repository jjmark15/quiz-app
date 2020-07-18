use thiserror::Error;
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::application::config::version::ApiVersion;
use crate::application::logging;
use crate::application::logging::{LogEntry, LogEntryKVP};
use crate::application::web::accept_header::ParseAcceptHeaderError;
use crate::application::web::error::WebErrorResponse;

#[derive(Debug, Eq, PartialEq, Error)]
pub(crate) enum ApiValidationError {
    #[error("api version {} is incorrect", .0.version())]
    WrongApiVersion(ApiVersion),
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
