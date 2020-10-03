use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;

mod accept_header;
mod error;
mod filters;
mod handlers;
mod rejection;
mod response;
mod version;

pub(crate) use response::*;

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
