mod server;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io::Write,
};

use reqwest::Error;
use server::mc_server::Server;
use server::ServerErrors;

#[derive(Deserialize, Serialize, Debug)]
struct PaperVersion {
    project_id: String,
    project_name: String,
    version_groups: Vec<String>,
    versions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PaperVersionCommits {
    project_id: String,
    project_name: String,
    builds: Vec<i64>,
    version: String,
}

#[derive(Default)]
pub struct McServerManager {
    directory: Option<String>,
    installations: HashMap<String, Server>,
    plugins: Vec<String>,
    cache_file: String,
    version_uri: HashMap<String, String>, // map of version:versionURI
}

impl McServerManager {
    pub fn new() -> Self {
        Self {
            directory: None,
            installations: HashMap::new(),
            plugins: vec![],
            ..Default::default()
        }
    }
    pub fn set_directory(mut self, directory: &str) -> Self {
        self.directory = Some(directory.to_string());
        self
    }

    pub fn set_cache_directory(mut self, file: &str) -> Self {
        self.cache_file = file.to_string();
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
    fn add_to_cache(&self,version: &str, uri: &str) {
        let mut file: std::fs::File = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.cache_file)
            .unwrap();
        writeln!(file, "{}:{}", version, uri);
    }
    pub async fn get_available_versions(&mut self) -> Result<HashMap<String, String>, Error> {
        let response = reqwest::get("https://api.papermc.io/v2/projects/paper/")
            .await?
            .json::<PaperVersion>()
            .await;
        match response {
            Ok(parsed) => {
                let cached_builds = std::fs::read_to_string(&self.cache_file)
                    .unwrap()
                    .lines()
                    .map(|line| {
                        let entry = line
                            .split_once(':')
                            .expect("Infallible state reached. Cant split db entry");
                        (entry.0.to_string(), entry.1.to_string())
                    })
                    .collect::<HashMap<String, String>>();
                // get latest version
                let latest_verison = parsed.versions.last().unwrap().clone();

                for version in parsed.versions {
                    if version == latest_verison {
                        continue;
                    }

                    if cached_builds.contains_key(&version) {
                        self.version_uri.insert(
                            version.clone(),
                            dbg!(cached_builds.get(&version).unwrap().to_string()),
                        );
                    } else {
                        let latest_commit = get_latest_commit(&version).await?;
                        let uri = dbg!(format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-1.20.4-{}.jar",
                                                version,
                                                latest_commit,
                                                latest_commit)
                                            );
                        self.add_to_cache(&version, &uri);
                        self.version_uri.insert(version, uri);
                    }
                }
                // get latest build from latest version anyways
                let latest_commit = get_latest_commit(&latest_verison).await?;
                let uri = dbg!(format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-1.20.4-{}.jar",
                                                latest_verison,
                                                latest_commit,
                                                latest_commit)
                                            );
                self.version_uri.insert(latest_verison, uri);
            }
            Err(e) => {
                return Err(dbg!(e));
            }
        }
        Ok(self.version_uri.clone())
    }
}


async fn get_latest_commit(version: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(dbg!(format!(
        "https://api.papermc.io/v2/projects/paper/versions/{}",
        version
    )))
    .await?
    .json::<PaperVersionCommits>()
    .await;

    match response {
        Ok(parsed) => {
            dbg!(&parsed);
            let final_commit = parsed.builds.last().unwrap();
            return Ok(final_commit.to_string());
        }
        Err(e) => {
            println!("Error parsing data {:?}", e);

            return Err(e);
        }
    }
}
