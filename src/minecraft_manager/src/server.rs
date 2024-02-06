use std::{ffi::OsString, fmt};

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    CreateOk,
    CreateFail(String),
    RunOk,
    RunFail(String),
    StopOk,
    StopFail(String),
}
pub mod mc_server {
    use std::{
        fmt::format,
        fs::File,
        io::{Cursor, Write},
        path::{Path, PathBuf},
    };

    use super::Status;
    use serde::{Deserialize, Serialize};
    use sysinfo::{Pid, Signal};
    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct Server {
        pub working_directory: String,
        pub properties_path: String,
        pub installed_plugins: Vec<String>,
        pub version: String,
        port: i64,
        pid: usize,
        pub active: bool, // we use this in the webserver, the run/stop process doesnt care about this beyond updating it
    }
    use std::process::Command;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
    impl Server {
        pub fn load(path: PathBuf) -> Self {
            serde_json::from_str(&std::fs::read_to_string(dbg!(path.join("server_data.json"))).unwrap())
                .unwrap()
        }
        pub async fn run_self(&mut self) -> Result<Status> {
            let working_dir = Path::new(&self.working_directory);
            let s = sysinfo::System::new_all();
            if let Some(process) = s.process(Pid::from(self.pid)) {
                if process.status() != sysinfo::ProcessStatus::Zombie {
                    return Ok(Status::RunFail("LMAO DYING WTF?".to_string()));
                };
            }

            Command::new("bash")
                .current_dir(working_dir)
                .arg("-c")
                .arg("mkfifo ./input_fifo -m=777")
                .output()
                .expect("Couldnt create fifo")
                .stdout;

            let command = Command::new("bash")
                .current_dir(working_dir)
                .arg("-c")
                .arg(format!("while [ 1 ] ; do cat input_fifo ; done | java -jar paper.jar --nogui --port {} > server_output",self.port))
                .spawn()
                .expect("failed to execute process");

            self.active = true;
            self.pid = command.id() as usize;
            self.update_config();

            Ok(Status::RunOk)
        }

        pub async fn stop_self(&mut self) -> Result<Status> {
            let s = System::new_all();
            if let Some(process) = s.process(Pid::from(self.pid)) {
                println!("{:?}", process.status());
            }
            let mut file: std::fs::File = std::fs::OpenOptions::new()
                .write(true)
                .open(Path::new(&self.working_directory).join("input_fifo"))
                .unwrap();

            let _ = writeln!(file, "stop");

            // WE NEED TO RELEASE THE PID FROM THE PROCESS TABLE
            use sysinfo::{Pid, System};

            if let Some(process) = s.process(Pid::from(self.pid)) {
                process.kill();
                process.wait();
            }
            self.active = false;
            self.update_config();
            Ok(Status::StopOk)
        }

        fn update_config(&self) {
            serde_json::to_writer_pretty(
                &File::create(Path::new(&self.working_directory).join("server_data.json")).unwrap(),
                &self,
            )
            .unwrap();
        }
        pub async fn create_new_server(
            server_name: &str,
            port: i64,
            install_directory: &str,
            url: &str,
        ) -> Result<Self> {
            let server_dir = Path::new(&install_directory).join(server_name);
            let _ = std::fs::create_dir(&server_dir);

            let _file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(server_dir.join("eula.txt"))
                .unwrap()
                .write_all(b"eula=true");

            let response = reqwest::get(url).await?;
            let mut file = std::fs::File::create(server_dir.join("paper.jar"))?;
            let mut content = Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;

            let x = Server {
                working_directory: server_dir.to_str().unwrap().to_string(),
                port,
                ..Default::default()
            };
            x.update_config();
            Ok(x)
        }
    }
}
