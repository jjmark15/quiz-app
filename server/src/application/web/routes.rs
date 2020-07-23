use warp::Filter;

use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::filters::{admin, api_filters};
use crate::application::web::rejection::handle_rejection;

pub fn routes<'a, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuizService: 'a + QuizServiceInterface<'a>,
{
    api_filters::<'a, QuizService>()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
