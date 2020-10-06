use log::debug;
use warp::http::StatusCode;
use warp::Rejection;

use structured_log_bodies::simple_log;

use crate::ports::http::response::{ErrorResponseMapper, WebErrorResponse};
use crate::ports::http::warp::filters::validate::api::error::ApiValidationError;
use crate::ports::logging::structured_log_bodies::RejectionLogEntity;

pub(crate) async fn handle_rejection(rej: Rejection) -> Result<impl warp::reply::Reply, Rejection> {
    let error_response_mapper = ErrorResponseMapper::new();
    if let Some(err) = rej.find::<ApiValidationError>() {
        debug!("{}", simple_log(RejectionLogEntity::from_error(err)));
        Ok(error_response_mapper.map(err))
    } else {
        Err(rej)
    }
}

impl WebErrorResponse for ApiValidationError {
    fn http_status_code(&self) -> StatusCode {
        StatusCode::NOT_ACCEPTABLE
    }
}
