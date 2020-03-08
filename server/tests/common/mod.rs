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
