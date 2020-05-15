use warp::Filter;

use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::filters::{admin, api_filters};
use crate::application::web::rejection::handle_rejection;

pub fn routes<
    QuestionSet: 'static + QuestionSetInterface<'static>,
    QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    api_filters::<QuestionSet, QuizService>()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
