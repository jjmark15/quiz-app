use warp::Filter;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod validate;

pub(crate) fn api_filters(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet())
}
