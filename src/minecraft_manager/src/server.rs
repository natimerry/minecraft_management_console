use std::{ffi::OsString, fmt};

#[derive(Debug)]
pub enum ServerErrors {
    FsError(String),
}
impl fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for ServerErrors {
    fn from(value: std::io::Error) -> Self {
        ServerErrors::FsError(format!("{}", value))
    }
}

impl From<OsString> for ServerErrors {
    fn from(value: OsString) -> Self {
        ServerErrors::FsError(format!("Unable to convert to UTF-8 string: {:?}", value))
    }
}



pub mod mc_server {
    use std::path::Path;

    #[derive(Default)]
    pub struct Server {
        pub is_active: bool,
        pub properties_path: Option<String>,
        pub server_jar_path: Option<String>,
        pub installed_plugins: Vec<String>,
    }

    impl Server {
        fn create_new_server(&mut self, server_name: String,install_directory:String){
            let _ = std::fs::create_dir(Path::new(&install_directory).join(server_name)); // create a new dir

        }
    }
}
