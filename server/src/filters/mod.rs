use warp::Filter;

pub mod greeting;
pub mod quiz;
pub mod validate;

pub fn app_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    validate::validate_api_version().and(greeting::greet().or(quiz::quiz()))
}
