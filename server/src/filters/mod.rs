use warp::Filter;

pub mod greeting;
pub mod quiz;

pub fn app_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    greeting::greet().or(quiz::quiz())
}
