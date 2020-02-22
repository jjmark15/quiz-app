use warp::Filter;

use crate::handlers::greeting;

pub fn greet() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::path("greeting")
        .and(warp::get())
        .and(warp::path("hello"))
        .and(hi().or(hi_you()))
}

pub fn hi() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(greeting::hello_world)
}

pub fn hi_you() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param().map(greeting::hello_name)
}
