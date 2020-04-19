use warp::Filter;

use quiz_domain::models::quiz::question::ModelIDInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz::QuestionSetReply;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod quiz;
pub(crate) mod validate;

pub(crate) fn api_filters<
    ID: 'static + ModelIDInterface<'static>,
    QuestionSet: 'static + QuestionSetReply<'static, ID>,
    QuizService: 'static + QuizServiceInterface<'static, ID, QuestionSet>,
>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz_routes::<ID, QuestionSet, QuizService>()))
}
