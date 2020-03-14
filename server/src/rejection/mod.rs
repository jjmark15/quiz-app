use std::convert::Infallible;

use log::{debug, warn};
use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::Rejection;

use crate::error::api_validation_error::ApiValidationError;
use crate::error::Error;
use crate::logging::{log_string, LogEntry, LogEntryKVP};

pub async fn handle_api_validation_error(
    rej: Rejection,
) -> Result<warp::reply::WithStatus<Json>, Infallible> {
    let error_message: ErrorMessage;
    let code: StatusCode;

    if rej.is_not_found() {
        code = StatusCode::NOT_FOUND;
        error_message = ErrorMessage::new(code.as_u16(), "NOT_FOUND".to_string());
    } else if let Some(err) = rej.find::<ApiValidationError>() {
        code = StatusCode::NOT_ACCEPTABLE;
        error_message = err.error_message();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        error_message = ErrorMessage::new(code.as_u16(), "UNHANDLED_REJECTION".to_string());
        warn!("{}", log_string(&error_message));
    }

    debug!("{}", log_string(&error_message));

    let json = warp::reply::json(&error_message);
    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub fn new(code: u16, message: String) -> ErrorMessage {
        ErrorMessage { code, message }
    }
}

impl LogEntry for ErrorMessage {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "rejection"),
            LogEntryKVP::new("code", format!("{}", self.code)),
            LogEntryKVP::new("message", &self.message),
        ]
    }
}
