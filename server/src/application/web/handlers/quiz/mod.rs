use std::sync::Arc;

use crate::application::web::handlers::quiz::reply::QuestionSetResponse;
use crate::application::ApplicationService;

mod reply;

pub(crate) async fn example_question_set(
    application_service: Arc<impl ApplicationService>,
) -> Result<QuestionSetResponse, warp::reject::Rejection> {
    Ok(application_service.get_example_question_set().into())
}
