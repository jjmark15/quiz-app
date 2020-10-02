use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;

pub(crate) mod admin;
pub(crate) mod greeting;
pub(crate) mod quiz;
pub(crate) mod validate;

pub(crate) fn api_filters<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::path("api")
        .and(
            warp::header::exact("accept", "*/*")
                .or(validate::api::version::valid_api_version())
                .unify(),
        )
        .and(greeting::greet().or(quiz::quiz_routes(application_service)))
}
