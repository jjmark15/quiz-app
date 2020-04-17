use warp::Filter;

use crate::application::web::handlers::quiz::QuestionSetReply;
use crate::domain::models::quiz::question::ModelIDInterface;
use crate::domain::services::quiz::QuizServiceInterface;

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
