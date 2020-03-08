use warp::{Filter, Rejection};

use crate::headers::get_application_header_prefix;

pub fn validate_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::header("accept")
        .and_then(|accept_string: String| async move {
            if accept_string.starts_with(&get_application_header_prefix()) {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one()
}
