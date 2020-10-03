use warp::Filter;

use crate::ports::http::warp_port::handlers::greeting;

pub(crate) fn greet() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path::path("greeting")
        .and(warp::get())
        .and(warp::path("hello"))
        .and(hi().or(hi_you()))
}

pub(crate) fn hi() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(greeting::hello_world)
}

pub(crate) fn hi_you() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param().map(greeting::hello_name)
}
