use warp::Filter;

use quiz_domain::models::quiz::{ModelID, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz;

pub(crate) fn quiz_routes<'a, QuestionSet, QuizService>(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'a
where
    QuestionSet: 'a + QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
    QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
{
    warp::path("quiz").and(example_question_set::<'a, QuestionSet, QuizService>())
}

fn example_question_set<'a, QuestionSet, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuestionSet: 'a + QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
    QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
{
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and_then(quiz::example_question_set::<'a, QuestionSet, QuizService>)
}
