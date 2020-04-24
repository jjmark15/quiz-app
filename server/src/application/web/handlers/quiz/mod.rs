use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz::reply::QuestionSetReply;

mod reply;

pub(crate) async fn example_question_set<
    'a,
    QuestionSet: QuestionSetInterface<'a>,
    QuizService: QuizServiceInterface<'a, QuestionSet>,
>() -> Result<QuestionSetReply, warp::reject::Rejection> {
    Ok(QuizService::get_example_question_set().into())
}
