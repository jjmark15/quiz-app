use quiz_domain::models::quiz::{ModelID, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz::reply::QuestionSetReply;

mod reply;

pub(crate) async fn example_question_set<'a, QuestionSet, QuizService>(
) -> Result<QuestionSetReply, warp::reject::Rejection>
where
    QuestionSet: 'a + QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
    QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
{
    Ok(QuizService::get_example_question_set().into())
}
