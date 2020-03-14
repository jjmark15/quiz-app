use std::convert::Infallible;

use warp::Filter;

use crate::rejection::handle_api_validation_error;

pub mod greeting;
pub mod quiz;
pub mod validate;

pub fn app_filters() -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api_version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz()))
        .recover(handle_api_validation_error)
}
