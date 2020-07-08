use crate::common::web::Endpoint;

pub(crate) fn get_request_url<S: AsRef<str>>(host: S, endpoint: Endpoint) -> String {
    format!("{}{}", host.as_ref(), endpoint.path_string())
}
