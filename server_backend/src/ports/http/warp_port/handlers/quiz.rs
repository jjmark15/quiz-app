use std::sync::Arc;

use crate::application::ApplicationService;
use crate::ports::http::warp_port::response::QuestionSetResponse;

pub(crate) async fn example_question_set(
    application_service: Arc<impl ApplicationService>,
) -> Result<QuestionSetResponse, warp::reject::Rejection> {
    Ok(application_service.get_example_question_set().into())
}
