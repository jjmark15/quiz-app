use serde::Serialize;
use warp::reply::Json;
use warp::Reply;

use crate::application::error::ApplicationError;
use crate::application::logging::{LogEntry, LogEntryKVP};
use crate::application::web::response::ErrorResponse;

#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub(crate) fn new(code: u16, message: String) -> ErrorMessage {
        ErrorMessage { code, message }
    }
}

impl Into<warp::reply::Json> for ErrorMessage {
    fn into(self) -> Json {
        warp::reply::json(&self)
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

pub(crate) trait WebError: ApplicationError {
    fn http_status_code(&self) -> warp::http::StatusCode;

    fn error_message(&self) -> ErrorMessage {
        ErrorMessage::new(self.http_status_code().as_u16(), format!("{}", self))
    }

    fn error_response(&self) -> ErrorResponse {
        ErrorResponse(
            warp::reply::with_status::<warp::reply::Json>(
                self.error_message().into(),
                self.http_status_code(),
            )
            .into_response(),
        )
    }
}
