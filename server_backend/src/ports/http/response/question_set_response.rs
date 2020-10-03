use serde::{Deserialize, Serialize};
use warp::reply::Response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetResponse {
    id: String,
    name: String,
}

impl QuestionSetResponse {
    pub fn new(id: String, name: String) -> Self {
        QuestionSetResponse { id, name }
    }
}

impl warp::Reply for QuestionSetResponse {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
