use quiz_domain::models::quiz::question::{ModelIDInterface, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

pub trait QuestionSetReply<'a, ID: ModelIDInterface<'a>>:
    QuestionSetInterface<'a, ID> + warp::Reply
{
}

impl<'a, ID: ModelIDInterface<'a>, QuestionSet: QuestionSetInterface<'a, ID> + warp::Reply>
    QuestionSetReply<'a, ID> for QuestionSet
{
}

pub(crate) async fn example_question_set<
    'a,
    ID: ModelIDInterface<'a>,
    QuestionSet: QuestionSetReply<'a, ID>,
    QuizService: QuizServiceInterface<'a, ID, QuestionSet>,
>() -> Result<QuestionSet, warp::reject::Rejection> {
    Ok(QuizService::get_example_question_set())
}
