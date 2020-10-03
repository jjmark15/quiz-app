use serde::{Deserialize, Serialize};
use warp::reply::Response;

use quiz_domain::{ModelID, QuestionSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetResponse {
    id: String,
    name: String,
}

impl<'a, QS> From<QS> for QuestionSetResponse
where
    QS: QuestionSet,
{
    fn from(q: QS) -> Self {
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
