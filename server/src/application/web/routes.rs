use warp::Filter;

use crate::application::web::filters::{admin, api_filters};
use crate::application::web::handlers::quiz::QuestionSetReply;
use crate::application::web::rejection::handle_rejection;
use crate::domain::models::quiz::question::ModelIDInterface;
use crate::domain::services::quiz::QuizServiceInterface;

pub fn routes<
    ID: 'static + ModelIDInterface<'static>,
    QuestionSet: 'static + QuestionSetReply<'static, ID>,
    QuizService: 'static + QuizServiceInterface<'static, ID, QuestionSet>,
>() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    api_filters::<ID, QuestionSet, QuizService>()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
