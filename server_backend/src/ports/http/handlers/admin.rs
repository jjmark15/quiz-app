use warp::reply::WithStatus;

pub(crate) fn status() -> WithStatus<&'static str> {
    warp::reply::with_status("OK", warp::http::StatusCode::OK)
}
