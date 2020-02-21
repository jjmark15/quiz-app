pub fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}

pub fn hello_world() -> &'static str {
    "Hello, World!"
}
