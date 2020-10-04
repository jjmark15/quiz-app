use warp::reply::Json;

use crate::ports::http::response::{ErrorResponse, ErrorResponseBody};

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status::<warp::reply::Json>(self.body().into(), self.status_code())
            .into_response()
    }
}

impl Into<Json> for ErrorResponseBody {
    fn into(self) -> Json {
        warp::reply::json(&self)
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use warp::Reply;

    use super::*;

    #[test]
    fn error_response_implements_reply() {
        let error_response_body = ErrorResponseBody::new("message".to_string());
        let error_response = ErrorResponse::new(error_response_body, StatusCode::BAD_REQUEST);
        let _: warp::reply::Response = error_response.into_response();
    }

    #[test]
    fn error_response_body_maps_to_json() {
        let error_response_body = ErrorResponseBody::new("message".to_string());
        let _: Json = error_response_body.into();
    }
}
