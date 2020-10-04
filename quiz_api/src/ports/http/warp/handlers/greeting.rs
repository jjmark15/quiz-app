pub(crate) fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}

pub(crate) fn hello_world() -> &'static str {
    "Hello, World!"
}
