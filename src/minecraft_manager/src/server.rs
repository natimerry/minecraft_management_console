use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io::Write,
};
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
    use std::{io::{Cursor, Write}, path::Path};
    #[derive(Default)]
    pub struct Server {
        pub is_active: bool,
        pub properties_path: Option<String>,
        pub server_jar_path: Option<String>,
        pub installed_plugins: Vec<String>,
    }
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
    impl Server {
        pub async fn create_new_server(
            &mut self,
            server_name: &str,
            install_directory: &str,
            url: &str,
        ) -> Result<()> {
            let server_dir = Path::new(&install_directory).join(server_name);
            let _ = std::fs::create_dir(&server_dir);

            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(server_dir.join("eula.txt"))
                .unwrap()
                .write_all(b"eula=true");

            let response = dbg!(reqwest::get(url).await?);
            let mut file = std::fs::File::create(server_dir.join("paper.jar"))?;
            let mut content = Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
            Ok(())
        }
    }
}
