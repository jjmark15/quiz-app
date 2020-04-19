use warp::Filter;

use quiz_domain::models::quiz::question::ModelIDInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz;
use crate::application::web::handlers::quiz::QuestionSetReply;

pub(crate) fn quiz_routes<
    ID: 'static + ModelIDInterface<'static>,
    QuestionSet: 'static + QuestionSetReply<'static, ID>,
    QuizService: 'static + QuizServiceInterface<'static, ID, QuestionSet>,
>() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("quiz").and(example_question_set::<ID, QuestionSet, QuizService>())
}

fn example_question_set<
    ID: 'static + ModelIDInterface<'static>,
    QuestionSet: 'static + QuestionSetReply<'static, ID>,
    QuizService: 'static + QuizServiceInterface<'static, ID, QuestionSet>,
>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and_then(quiz::example_question_set::<ID, QuestionSet, QuizService>)
}
