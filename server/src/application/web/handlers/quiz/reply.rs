use serde::{Deserialize, Serialize};
use warp::reply::Response;

use quiz_domain::models::quiz::{ModelID, QuestionSetInterface};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionSetReply {
    id: String,
    name: String,
}

impl<'a, QuestionSet> From<QuestionSet> for QuestionSetReply
where
    QuestionSet: QuestionSetInterface<'a>,
{
    fn from(q: QuestionSet) -> Self {
        QuestionSetReply {
            id: q.id().value().to_string(),
            name: q.name().clone(),
        }
    }
}

impl warp::Reply for QuestionSetReply {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
