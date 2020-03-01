use warp::Filter;

use crate::config::version::ApiVersion;

pub mod greeting;
pub mod quiz;
pub mod validate;

pub fn app_filters(
    api_version: ApiVersion,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    validate::api_version(api_version)
        .recover(validate::handle_rejection)
        .and(greeting::greet().or(quiz::quiz()))
}
