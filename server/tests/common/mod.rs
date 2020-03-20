use warp::test::{request, RequestBuilder};

pub fn get_request_endpoint_string(partial_endpoint: &str) -> String {
    let trimmed_partial = {
        if partial_endpoint.starts_with('/') {
            partial_endpoint.get(1..).unwrap()
        } else {
            partial_endpoint
        }
    };
    vec!["/api".to_string(), trimmed_partial.to_string()].join("/")
}

pub fn default_application_accept_header() -> &'static str {
    "application/vnd.warpj.v0"
}

pub fn default_request_builder() -> RequestBuilder {
    request().header("Accept", default_application_accept_header())
}
