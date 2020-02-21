use warp::Filter;

pub fn quiz() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::path("quiz").and(warp::get()).map(|| "quiz fun")
}
