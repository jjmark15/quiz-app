use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;

mod error;
mod filters;
mod handlers;
mod rejection;
pub mod response;

pub fn routes<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    filters::api_filters(application_service)
        .or(filters::admin::admin_filters())
        .recover(rejection::handle_rejection)
}
