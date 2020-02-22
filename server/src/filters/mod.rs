use warp::Filter;

use crate::config::version::ApiVersion;

pub mod greeting;
pub mod quiz;
pub mod validate;

pub fn app_filters(
    api_version: ApiVersion,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api").and(warp::path(api_version.version_string().to_string()));
    api_prefix.and(greeting::greet().or(quiz::quiz()))
}
