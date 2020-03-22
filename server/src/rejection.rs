use std::convert::Infallible;

use log::{debug, warn};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::error::{Error, ErrorMessage};
use crate::filters::validate::api_version::error::ApiValidationError;
use crate::logging::log_string;
use crate::response::ErrorResponse;

pub(crate) async fn handle_rejection(
    rej: Rejection,
) -> Result<impl warp::reply::Reply, Infallible> {
    let reply;

    if let Some(err) = rej.find::<ApiValidationError>() {
        reply = err.error_response();
        debug!("{}", log_string(err));
    } else {
        let error_message = ErrorMessage::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "UNHANDLED_REJECTION".to_string(),
        );
        reply = ErrorResponse(
            warp::reply::with_status(
                warp::reply::json(&error_message),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response(),
        );
        warn!("{}", log_string(&error_message));
    }

    Ok(reply)
}
