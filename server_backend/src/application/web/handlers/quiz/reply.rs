use serde::{Deserialize, Serialize};
use warp::reply::Response;

use quiz_domain::{ModelID, QuestionSetInterface};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetResponse {
    id: String,
    name: String,
}

impl<'a, QuestionSet> From<QuestionSet> for QuestionSetResponse
where
    QuestionSet: QuestionSetInterface,
{
    fn from(q: QuestionSet) -> Self {
        QuestionSetResponse {
            id: q.id().value().to_string(),
            name: q.name().clone(),
        }
    }
}

impl warp::Reply for QuestionSetResponse {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
