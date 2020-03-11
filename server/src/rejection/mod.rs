use std::convert::Infallible;

use serde::Serialize;
use warp::reply::Json;
use warp::Rejection;

pub async fn handle_api_validation_error(
    _r: Rejection,
) -> Result<warp::reply::WithStatus<Json>, Infallible> {
    println!("got here"); // FIXME: remove this
    let code: warp::http::StatusCode;
    let message: String;

    // match r.find::<ApiValidationError>() {
    //     Some(e) => {
    //         code = warp::http::StatusCode::NOT_ACCEPTABLE;
    //         message = e.description();
    //     }
    //     None => {
    //         log::error!("unhandled server error");
    //         code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
    //         message = ApiValidationError::new(ApiValidationErrorKind::Unknown).description();
    //     }
    // }
    message = "wut".to_string();
    code = warp::http::StatusCode::IM_A_TEAPOT;
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });
    println!("got here");
    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}
