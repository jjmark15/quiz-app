use std::sync::Arc;

use warp::Filter;

use crate::application::ApplicationService;
use crate::ports::http::filters::{admin, api_filters};
use crate::ports::http::rejection::handle_rejection;

pub fn routes<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    api_filters(application_service)
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
