use serde::{Deserialize, Serialize};
use warp::reply::Response;

use quiz_domain::models::quiz::question::QuestionSetInterface;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetReply {
    id: String,
    name: String,
}

impl<'a, QuestionSet: QuestionSetInterface<'a>> From<QuestionSet> for QuestionSetReply {
    fn from(q: QuestionSet) -> Self {
        QuestionSetReply {
            id: q.id().value(),
            name: q.name().clone(),
        }
    }
}

impl warp::Reply for QuestionSetReply {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
