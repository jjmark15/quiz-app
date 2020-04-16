use std::fmt;
use std::fmt::Display;

use serde::export::Formatter;
use warp::http::StatusCode;

use crate::application::logging::LogEntryKVP;
use crate::application::web::error::Error as WebError;

#[derive(Debug)]
pub(crate) enum Error {
    InvalidApplicationProfile(String),
    ApplicationProfileEnvNotSet,
    InvalidValueOverride,
    ValueOverrideEnvNotSet,
}

impl WebError for Error {
    fn description(&self) -> String {
        match self {
            Error::InvalidApplicationProfile(p) => {
                format!("received invalid application profile string: {}", p)
            }
            Error::ApplicationProfileEnvNotSet => {
                String::from("application profile environment variable is not set")
            }
            Error::InvalidValueOverride => {
                String::from("received invalid environment variable override")
            }
            Error::ValueOverrideEnvNotSet => {
                String::from("value override environment variable is not set")
            }
        }
    }

    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl crate::application::logging::LogEntry for Error {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "error"),
            LogEntryKVP::new("description", self.description()),
        ]
    }
}

impl std::error::Error for Error {}
