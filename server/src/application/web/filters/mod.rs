use warp::Filter;

use quiz_domain::models::quiz::{ModelID, QuestionSetInterface};
use quiz_domain::services::quiz::QuizServiceInterface;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod quiz;
pub(crate) mod validate;

pub(crate) fn api_filters<'a, QuestionSet, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuestionSet: 'a + QuestionSetInterface<'a>,
    QuestionSet::ID: ModelID<'a>,
    QuizService: 'a + QuizServiceInterface<'a, QuestionSet>,
{
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz_routes::<'a, QuestionSet, QuizService>()))
}
