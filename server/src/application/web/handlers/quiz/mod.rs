use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz::reply::QuestionSetReply;

mod reply;

pub(crate) async fn example_question_set<'a, QuizService>(
) -> Result<QuestionSetReply, warp::reject::Rejection>
where
    QuizService: 'a + QuizServiceInterface<'a>,
{
    Ok(QuizService::get_example_question_set().into())
}
