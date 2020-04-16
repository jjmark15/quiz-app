use core::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use warp::http::StatusCode;
use warp::reject::Reject;

use crate::config::version::ApiVersion;
use crate::logging;
use crate::logging::{LogEntry, LogEntryKVP};
use crate::web::error::Error as WebError;

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
        WebError::description(self).fmt(f)
    }
}

impl Reject for ApiValidationError {}

impl crate::web::error::Error for ApiValidationError {
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

    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
}

impl LogEntry for ApiValidationError {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            logging::LogEntryKVP::new("type", "error"),
            logging::LogEntryKVP::new("message", WebError::description(self)),
        ]
    }
}
