use thiserror::Error;
use warp::http::StatusCode;

use crate::application::error::ApplicationError;
use crate::application::logging::LogEntryKVP;
use crate::application::web::error::WebError;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("received invalid application profile string: {0}")]
    InvalidApplicationProfile(String),
    #[error("application profile environment variable is not set")]
    ApplicationProfileEnvNotSet,
    #[error("received invalid environment variable override")]
    InvalidValueOverride,
    #[error("value override environment variable is not set")]
    ValueOverrideEnvNotSet,
}

impl WebError for Error {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}

impl ApplicationError for Error {}

impl crate::application::logging::LogEntry for Error {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "error"),
            LogEntryKVP::new("description", format!("{}", self)),
        ]
    }
}
