use warp::Filter;

use quiz_domain::models::quiz::question::QuestionSetInterface;
use quiz_domain::services::quiz::QuizServiceInterface;

use crate::application::web::handlers::quiz;

pub(crate) fn quiz_routes<
    QuestionSet: 'static + QuestionSetInterface<'static>,
    QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
>() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("quiz").and(example_question_set::<QuestionSet, QuizService>())
}

fn example_question_set<
    QuestionSet: 'static + QuestionSetInterface<'static>,
    QuizService: 'static + QuizServiceInterface<'static, QuestionSet>,
>() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and_then(quiz::example_question_set::<QuestionSet, QuizService>)
}
