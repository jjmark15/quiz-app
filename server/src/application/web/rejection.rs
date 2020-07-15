use log::debug;
use warp::Rejection;

use crate::application::logging::log_string;
use crate::application::web::error::WebErrorResponse;
use crate::application::web::filters::validate::api::error::ApiValidationError;

pub(crate) async fn handle_rejection(rej: Rejection) -> Result<impl warp::reply::Reply, Rejection> {
    if let Some(err) = rej.find::<ApiValidationError>() {
        debug!("{}", log_string(err));
        Ok(err.error_response())
    } else {
        Err(rej)
    }
}
