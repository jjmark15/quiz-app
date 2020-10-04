use thiserror::Error;
use warp::reject::Reject;

use crate::ports::http::accept_header::ParseAcceptHeaderError;
use crate::ports::http::api_version::{ApiVersion, ApiVersionImpl};

#[derive(Debug, Eq, PartialEq, Error)]
pub(crate) enum ApiValidationError {
    #[error("api version {} is incorrect", .0.version())]
    WrongApiVersion(ApiVersionImpl),
    #[error(transparent)]
    InvalidAcceptHeader(#[from] ParseAcceptHeaderError),
}

impl Reject for ApiValidationError {}
