use std::sync::Arc;

use warp::Filter;

use crate::application::web::handlers::quiz;
use crate::application::ApplicationService;

pub(crate) fn quiz_routes<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::path("quiz").and(example_question_set(application_service))
}

fn example_question_set<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    let app_service = warp::any().map(move || application_service.clone());
    warp::get()
        .and(warp::path("question"))
        .and(warp::path("set"))
        .and(warp::path("example"))
        .and(app_service)
        .and_then(quiz::example_question_set)
}
