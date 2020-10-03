use crate::ports::http::QuestionSetResponse;
use quiz_domain::{ModelID, QuestionSet};

impl<'a, QS> From<QS> for QuestionSetResponse
where
    QS: QuestionSet,
{
    fn from(q: QS) -> Self {
        QuestionSetResponse::new(q.id().value().to_string(), q.name().clone())
    }
}
