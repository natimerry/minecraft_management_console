use std::path::Path;

pub struct Server{
    is_active: bool,
    properties_path: Option<String>,
    server_jar_path: Option<String>,
    installed_plugins: Vec<String>,
}