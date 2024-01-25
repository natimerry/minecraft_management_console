pub (crate) mod mc_server {
    use core::fmt;
    use std::ffi::OsString;
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

    #[derive(Default)]
    pub struct Server {
        pub is_active: bool,
        pub properties_path: Option<String>,
        pub server_jar_path: Option<String>,
        pub installed_plugins: Vec<String>,
    }

    impl Server {}
}
