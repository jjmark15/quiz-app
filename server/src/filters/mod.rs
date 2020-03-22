use std::convert::Infallible;

use warp::Filter;

use crate::rejection::handle_rejection;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod validate;

fn api_filters() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api_version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet())
}

pub fn app_filters() -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    api_filters()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
