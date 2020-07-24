use warp::Filter;

use quiz_domain::services::QuizServiceInterface;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod quiz;
pub(crate) mod validate;

pub(crate) fn api_filters<'a, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuizService: 'a + QuizServiceInterface<'a>,
{
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz_routes::<'a, QuizService>()))
}
