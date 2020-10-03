use log::debug;
use warp::Rejection;

use crate::ports::http::warp_port::error::WebErrorResponse;
use crate::ports::http::warp_port::filters::validate::api::error::ApiValidationError;
use crate::ports::logging::log_string;

pub(crate) async fn handle_rejection(rej: Rejection) -> Result<impl warp::reply::Reply, Rejection> {
    if let Some(err) = rej.find::<ApiValidationError>() {
        debug!("{}", log_string(err));
        Ok(err.error_response())
    } else {
        Err(rej)
    }
}
