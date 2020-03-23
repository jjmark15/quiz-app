use std::convert::Infallible;

use warp::Filter;

use crate::filters::{admin, api_filters};
use crate::rejection::handle_rejection;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    api_filters()
        .or(admin::admin_filters())
        .recover(handle_rejection)
}
