use warp::reply::Response;

use crate::ports::http::response::QuestionSetResponse;

impl warp::Reply for QuestionSetResponse {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
