use log::debug;
use warp::{Filter, Rejection};

use crate::ports::http::accept_header::AcceptHeader;
use crate::ports::http::api_version::{ApiVersion, ApiVersionImpl};
use crate::ports::http::warp::filters::validate::api::error::ApiValidationError;

pub(crate) fn valid_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::optional::<String>("accept")
        .and_then(validate_api_version)
        .untuple_one()
}

async fn validate_api_version(optional_accept_string: Option<String>) -> Result<(), Rejection> {
    if let Some(accept_string) = optional_accept_string {
        debug!("accept header is present");
        match AcceptHeader::parse(accept_string) {
            Ok(accept_header) => {
                let api_version: &ApiVersionImpl = accept_header.api_version();
                if !api_version.is_latest() {
                    return handle_old_api_version(*api_version);
                }
                Ok(())
            }
            Err(err) => handle_failed_api_version_extraction(err.into()),
        }
    } else {
        debug!("accept header is NOT present");
        Ok(())
    }
}

fn handle_old_api_version(version: ApiVersionImpl) -> Result<(), Rejection> {
    let err: ApiValidationError = ApiValidationError::WrongApiVersion(version);
    Err(warp::reject::custom(err))
}

fn handle_failed_api_version_extraction(err: ApiValidationError) -> Result<(), Rejection> {
    Err(warp::reject::custom(err))
}
