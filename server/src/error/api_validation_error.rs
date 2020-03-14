use core::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use warp::http::StatusCode;
use warp::reject::Reject;

use crate::error::Error;
use crate::logging;
use crate::rejection::ErrorMessage;

#[derive(Debug)]
pub struct ApiValidationError {
    kind: ApiValidationErrorKind,
    cause: Option<String>,
}

impl ApiValidationError {
    #[cfg(test)]
    pub fn kind(&self) -> ApiValidationErrorKind {
        self.kind
    }

    pub fn new(kind: ApiValidationErrorKind) -> ApiValidationError {
        ApiValidationError { kind, cause: None }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum ApiValidationErrorKind {
    MissingMatch,
    UnableToParse,
    WrongApiVersion,
}

impl From<ParseIntError> for ApiValidationError {
    fn from(p: ParseIntError) -> Self {
        ApiValidationError {
            kind: ApiValidationErrorKind::UnableToParse,
            cause: Some(p.to_string()),
        }
    }
}

impl From<&ApiValidationError> for ApiValidationError {
    fn from(original: &ApiValidationError) -> Self {
        ApiValidationError {
            kind: original.kind,
            cause: original.cause.clone(),
        }
    }
}

impl std::error::Error for ApiValidationError {}

impl Display for ApiValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Reject for ApiValidationError {}

impl crate::error::Error for ApiValidationError {
    fn description(&self) -> String {
        let kind_description = match self.kind {
            ApiValidationErrorKind::MissingMatch => {
                "could not find an api version in accept header"
            }
            ApiValidationErrorKind::UnableToParse => {
                "api version in accept header could not be parsed"
            }
            ApiValidationErrorKind::WrongApiVersion => "api version is incorrect",
        };

        match &self.cause {
            Some(s) => format!("{} : {}", kind_description, s),
            None => kind_description.to_string(),
        }
    }

    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }

    fn error_message(&self) -> ErrorMessage {
        ErrorMessage::new(self.http_status_code().as_u16(), self.description())
    }
}

impl logging::LogEntry for ApiValidationError {
    fn log_entry_kvps(&self) -> Vec<logging::LogEntryKVP> {
        vec![
            logging::LogEntryKVP::new("type", "error"),
            logging::LogEntryKVP::new("kind", format!("ApiValidationError::{:?}", self.kind)),
            logging::LogEntryKVP::new("message", self.description()),
        ]
    }
}
