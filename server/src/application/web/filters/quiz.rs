use warp::Filter;

use crate::application::web::handlers::quiz;

pub(crate) fn quiz_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("quiz").and(example_question_set())
}

fn example_question_set(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and_then(quiz::example_question_set)
}
