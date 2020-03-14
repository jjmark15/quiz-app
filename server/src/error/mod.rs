use crate::logging::LogEntry;
use crate::rejection::ErrorMessage;

pub(crate) mod api_validation_error;

pub trait Error: std::error::Error + LogEntry {
    fn description(&self) -> String;

    fn http_status_code(&self) -> warp::http::StatusCode;

    fn error_message(&self) -> ErrorMessage;
}
