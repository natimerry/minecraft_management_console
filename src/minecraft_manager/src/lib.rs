pub mod server;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io::Write,
    path::{Path, PathBuf},
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

#[derive(Default, Deserialize, Serialize)]
pub struct McServerManager {
    #[serde(skip)]
    json_location: String,

    directory: String,
    installations: Vec<String>,
    cache_file: String,
    version_uri: HashMap<String, String>,
}

impl McServerManager {

    pub fn new(json_file: Option<&str>) -> Self {
        let json_path: PathBuf;
        match json_file {
            Some(path) => json_path = Path::new(&path).to_path_buf(),
            None => json_path = Path::new("./").join("mc_server_manager.json").to_path_buf(),
        }

        let mut manager: McServerManager;
        if json_path.exists() {
            manager = serde_json::from_str(&std::fs::read_to_string(&json_path).unwrap()).unwrap();
            manager.json_location = json_path.to_str().unwrap().to_string();
        } else {
            let current_dir = fs::canonicalize(Path::new("./")).unwrap();
            let cache_file = current_dir.join("cache.txt").to_str().unwrap().to_string();
            let directory = current_dir.join("minecraft_servers").to_str().unwrap().to_string();

            manager = Self {
                cache_file,
                directory,
                json_location: json_path.to_str().unwrap().to_string(),
                installations: vec![],
                ..Default::default()
            };

            let json_data = serde_json::to_string_pretty(&manager).unwrap();
            let _ = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(json_path)
                .unwrap()
                .write_all(format!("{json_data}").as_bytes());
        }
        if !Path::new(&manager.directory).exists() {
            std::fs::create_dir(&manager.directory);
        }
        manager
    }

    pub async fn create_new_server(&mut self, version: &str, name: &str) {
        let x = self.get_available_versions().await.unwrap();
        let url = x.get(version).unwrap();

        let _ = Server::create_new_server(name, &self.directory.clone(), url).await;

        let mut _file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(
                Path::new(&self.directory.clone())
                    .join(name)
                    .join("version.txt"),
            )
            .unwrap()
            .write_all(format!("{version}").as_bytes());
    }

    fn update_installations(&mut self) -> Result<(), ServerErrors> {
        let paths: ReadDir = fs::read_dir(&self.directory)?;

        for path in paths {
            let directory = path?;
            // let new_server = Server::new(directory.path());
            let new_server: Server = Server::load(directory.path());
            self.installations
                .push(directory.path().into_os_string().into_string()?);
        }
        Ok(())
    }
    pub fn get_installations(&mut self) -> Result<Vec<(String, String)>, ServerErrors> {
        self.update_installations()?;

        Ok(self
            .installations
            .iter()
            .map(|server| {
                let server_path = Path::new(server);
                (
                    server_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    std::fs::read_to_string(server_path.join("version.txt")).unwrap(),
                )
            })
            .collect::<Vec<(String, String)>>())
    }
    fn add_to_cache(&self, version: &str, uri: &str) {
        let mut file: std::fs::File = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.cache_file)
            .unwrap();
        let _ = writeln!(file, "{}:{}", version, uri);
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
                            cached_builds.get(&version).unwrap().to_string(),
                        );
                    } else {
                        let latest_commit = get_latest_commit(&version).await?;
                        let uri = dbg!(
                            format!(
            "https://api.papermc.io/v2/projects/paper/versions/{version}/builds/{latest_commit}/downloads/paper-{version}-{latest_commit}.jar"
                                ));
                        self.add_to_cache(&version, &uri);
                        self.version_uri.insert(version, uri);
                    }
                }
                // get latest build from latest version anyways
                let latest_commit = get_latest_commit(&latest_verison).await?;
                let uri = dbg!(format!(
            "https://api.papermc.io/v2/projects/paper/versions/{latest_verison}/builds/{latest_commit}/downloads/paper-{latest_verison}-{latest_commit}.jar",
                                            ));
                self.version_uri.insert(latest_verison, uri);
            }
            Err(e) => {
                return Err(dbg!(e));
            }
        }
        Ok(self.version_uri.clone())
    }

    pub async fn run_server(&self, name: &str) {
        let mut server = self.load_server(name);

        dbg!(server.run_self().await.unwrap());
    }


    pub async fn stop_server(&self, name: &str) {
        let mut server = self.load_server(name);

        dbg!(server.stop_self().await.unwrap());
    }

    fn load_server(&self,name: &str) -> Server{
        let workingdir = Path::new(&self.directory.clone()).join(name);
        Server::load(workingdir)
    }
    pub fn get_server_status(&self,name: &str) -> bool{
        let server = self.load_server(name);
        server.active
    }


}

async fn get_latest_commit(version: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(format!(
        "https://api.papermc.io/v2/projects/paper/versions/{}",
        version
    ))
    .await?
    .json::<PaperVersionCommits>()
    .await;

    match response {
        Ok(parsed) => {
            &parsed;
            let final_commit = parsed.builds.last().unwrap();
            return Ok(final_commit.to_string());
        }
        Err(e) => {
            println!("Error parsing data {:?}", e);

            return Err(e);
        }
    }
}
