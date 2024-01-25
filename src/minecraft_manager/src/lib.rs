mod server;
use std::{
    collections::HashMap,
    default,
    fs::{self, ReadDir},
};

use server::mc_server::{Server, ServerErrors};

#[derive(Default)]
pub struct McServerManager {
    directory: Option<String>,
    installations: HashMap<String, Server>,
    plugins: Vec<String>,
}

impl McServerManager {
    pub fn new() -> Self {
        Self {
            directory: None,
            installations: HashMap::new(),
            plugins: vec![],
        }
    }
    pub fn set_directory(mut self, directory: String) -> Self {
        self.directory = Some(directory);
        self
    }

    fn update_isntallations(&mut self) -> Result<(), ServerErrors> {
        let paths: ReadDir;
        match &self.directory {
            Some(dir) => paths = fs::read_dir(&dir)?,
            None => paths = fs::read_dir("./mc")?,
        }

        for path in paths {
            let directory = dbg!(path?);
            let files_in_dir = fs::read_dir(directory.path())?;
            let mut new_server = Server {
                ..Server::default()
            };
            for file_path in files_in_dir {
                let file_path_string = file_path?.path().into_os_string().into_string()?;

                let file = dbg!(file_path_string.rsplit("/").collect::<Vec<&str>>()[0]);

                match file {
                    "paper.jar" => new_server.server_jar_path = Some(file_path_string.clone()),
                    "server.properties" => {
                        new_server.properties_path = Some(file_path_string.clone())
                    }
                    ".lock" => new_server.is_active = true,
                    _ => (),
                }
            }

            self.installations
                .insert(directory.path().into_os_string().into_string()?, new_server);
        }
        Ok(())
    }
    pub fn get_installations(&mut self) -> Result<Vec<String>, ServerErrors> {
        self.update_isntallations()?;
        Ok(self
            .installations
            .keys()
            .map(|entry| entry.rsplit("/").collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>())
    }
}
