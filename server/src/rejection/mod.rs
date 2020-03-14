use std::convert::Infallible;

use log::{debug, warn};
use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::Rejection;

use crate::filters::validate::api_version::ApiValidationError;
use crate::logging;

pub async fn handle_api_validation_error(
    rej: Rejection,
) -> Result<warp::reply::WithStatus<Json>, Infallible> {
    let code: warp::http::StatusCode;
    let message: String;

    if rej.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(err) = rej.find::<ApiValidationError>() {
        debug!("{}", logging::log_string(err));
        code = StatusCode::NOT_ACCEPTABLE;
        message = err.description();
    } else {
        warn!("unhandled rejection: {:?}", rej);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });
    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}
