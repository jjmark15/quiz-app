use thiserror::Error;
use warp::http::StatusCode;

use crate::application::logging::LogEntryKVP;
use crate::application::web::error::WebErrorResponse;

#[derive(Debug, Error)]
pub(crate) enum ConfigError {
    #[error("received invalid application profile string: {0}")]
    InvalidApplicationProfile(String),
    #[error("application profile environment variable is not set")]
    ApplicationProfileEnvNotSet,
    #[error("received invalid environment variable override")]
    InvalidValueOverride,
    #[error("value override environment variable is not set")]
    ValueOverrideEnvNotSet,
}

impl WebErrorResponse for ConfigError {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}

impl crate::application::logging::LogEntry for ConfigError {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "error"),
            LogEntryKVP::new("description", format!("{}", self)),
        ]
    }
}
