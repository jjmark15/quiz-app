use std::num::ParseIntError;

use thiserror::Error;
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::application::config::version::ApiVersion;
use crate::application::logging;
use crate::application::logging::{LogEntry, LogEntryKVP};
use crate::application::web::error::WebErrorResponse;

#[derive(Debug, Eq, PartialEq, Error)]
pub(crate) enum ApiValidationError {
    #[error("could not find an api version in accept header")]
    MissingMatch,
    #[error("api version could not be parsed as {0}")]
    UnableToParse(#[from] ParseIntError),
    #[error("api version {} is incorrect", .0.version())]
    WrongApiVersion(ApiVersion),
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
