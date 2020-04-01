use warp::Filter;

use crate::web::filters::{admin, api_filters};
use crate::web::rejection::handle_rejection;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    api_filters()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
