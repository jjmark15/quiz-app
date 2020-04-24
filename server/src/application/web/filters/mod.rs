use warp::Filter;

use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod quiz;
pub(crate) mod validate;

pub(crate) fn api_filters<
    QuestionSet: 'static + QuestionSetInterface<'static>,
    QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz_routes::<QuestionSet, QuizService>()))
}
