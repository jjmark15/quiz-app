use warp::reply::Response;

use crate::application::services::quiz::QuizService;
use crate::domain::quiz::question::QuestionSet;

pub(crate) async fn example_question_set() -> Result<QuestionSet, warp::reject::Rejection> {
    Ok(QuizService::get_example_question_set())
}

impl warp::Reply for QuestionSet {
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}
