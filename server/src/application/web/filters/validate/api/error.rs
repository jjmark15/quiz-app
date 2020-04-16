use core::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use warp::http::StatusCode;
use warp::reject::Reject;

use crate::application::config::version::ApiVersion;
use crate::application::error::ApplicationError;
use crate::application::logging;
use crate::application::logging::{LogEntry, LogEntryKVP};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ApiValidationError {
    MissingMatch,
    UnableToParse(ParseIntError),
    WrongApiVersion(ApiVersion),
}

impl From<ParseIntError> for ApiValidationError {
    fn from(p: ParseIntError) -> Self {
        ApiValidationError::UnableToParse(p)
    }
}

impl std::error::Error for ApiValidationError {}

impl Display for ApiValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        ApplicationError::description(self).fmt(f)
    }
}

impl Reject for ApiValidationError {}

impl crate::application::web::error::WebError for ApiValidationError {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
}

impl ApplicationError for ApiValidationError {
    fn description(&self) -> String {
        match self {
            ApiValidationError::MissingMatch => {
                "could not find an api version in accept header".to_string()
            }
            ApiValidationError::UnableToParse(p) => {
                format!("api version could not be parsed as {}", p)
            }
            ApiValidationError::WrongApiVersion(v) => {
                format!("api version {} is incorrect", v.version())
            }
        }
    }
}

impl LogEntry for ApiValidationError {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            logging::LogEntryKVP::new("type", "error"),
            logging::LogEntryKVP::new("message", ApplicationError::description(self)),
        ]
    }
}
