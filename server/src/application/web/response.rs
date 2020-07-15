use serde::Serialize;
use warp::reply::Json;

pub(crate) struct ErrorResponse(pub(crate) warp::reply::Response);

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}

#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub(crate) fn new(code: u16, message: String) -> ErrorMessage {
        ErrorMessage { code, message }
    }

    pub(crate) fn code(&self) -> u16 {
        self.code
    }

    pub(crate) fn message(&self) -> &String {
        &self.message
    }
}

impl Into<warp::reply::Json> for ErrorMessage {
    fn into(self) -> Json {
        warp::reply::json(&self)
    }
}
