use warp::Filter;

use quiz_domain::models::quiz::question::{ModelID, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::filters::{admin, api_filters};
use crate::application::web::rejection::handle_rejection;

pub fn routes<'a, QuestionSet, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuestionSet: 'a + QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
    QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
{
    api_filters::<'a, QuestionSet, QuizService>()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
