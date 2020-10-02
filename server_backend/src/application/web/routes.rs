use std::sync::Arc;

use warp::Filter;

use crate::application::web::filters::{admin, api_filters};
use crate::application::web::rejection::handle_rejection;
use crate::application::ApplicationService;

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
