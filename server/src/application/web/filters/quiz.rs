use warp::Filter;

use quiz_domain::services::QuizServiceInterface;

use crate::application::web::handlers::quiz;

pub(crate) fn quiz_routes<'a, QuizService>(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'a
where
    QuizService: 'a + QuizServiceInterface,
{
    warp::path("quiz").and(example_question_set::<QuizService>())
}

fn example_question_set<'a, QuizService>(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a
where
    QuizService: 'a + QuizServiceInterface,
{
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and_then(quiz::example_question_set::<QuizService>)
}
