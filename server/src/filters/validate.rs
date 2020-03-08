use warp::{Filter, Rejection};

use crate::config::version::ApiVersion;

pub(crate) fn validate_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::header("accept")
        .and_then(|accept_string: String| async move {
            if accept_string.starts_with(ApiVersion::version_string()) {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one()
}
