use std::fmt;
use std::fmt::Display;

use serde::export::Formatter;
use warp::http::StatusCode;

use crate::error::Error as CrateError;
use crate::logging::LogEntryKVP;

#[derive(Debug)]
pub(crate) enum Error {
    InvalidProfile,
    ProfileEnvNotSet,
    InvalidVariableOverride,
    ValueOverrideEnvNotSet,
}

impl CrateError for Error {
    fn description(&self) -> String {
        match self {
            Error::InvalidProfile => String::from("received invalid application profile string"),
            Error::ProfileEnvNotSet => {
                String::from("application profile environment variable is not set")
            }
            Error::InvalidVariableOverride => {
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

impl crate::logging::LogEntry for Error {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "error"),
            LogEntryKVP::new("description", self.description()),
        ]
    }
}

impl std::error::Error for Error {}
