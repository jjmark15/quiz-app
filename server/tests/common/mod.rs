use server::config::version::ApiVersion;

pub fn get_request_endpoint_string(partial_endpoint: &str) -> String {
    let trimmed_partial = {
        if partial_endpoint.starts_with('/') {
            partial_endpoint.get(1..).unwrap()
        } else {
            partial_endpoint
        }
    };
    vec![
        format!(
            "/api/{api_version}",
            api_version = ApiVersion::latest().version_string()
        ),
        trimmed_partial.to_string(),
    ]
    .join("/")
}
