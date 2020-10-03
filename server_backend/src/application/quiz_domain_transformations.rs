use quiz_domain::{ModelID, QuestionSet};

use crate::ports::http::warp_port::response::QuestionSetResponse;

impl<'a, QS> From<QS> for QuestionSetResponse
where
    QS: QuestionSet,
{
    fn from(q: QS) -> Self {
        QuestionSetResponse::new(q.id().value().to_string(), q.name().clone())
    }
}
