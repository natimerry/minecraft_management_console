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
#[derive(Debug,Serialize,Deserialize)]
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
        fs::File,
        io::{Cursor, Write},
        os::unix::raw::pid_t,
        path::{Path, PathBuf},
    };

    use sysinfo::Signal;

    use super::Status;
    #[derive(Default, Debug)]
    pub struct Server {
        pub working_directory: String,
        pub properties_path: Option<String>,
        pub installed_plugins: Vec<String>,
    }
    use std::process::Command;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
    impl Server {
        pub async fn run_self(&mut self) -> Result<Status> {
            let working_dir = Path::new(&self.working_directory);
            let pid_file = working_dir.join("process_id");

            let mut pid: u32 = 0;
            if pid_file.exists() {
                let p = dbg!(std::fs::read_to_string(&pid_file)).unwrap();
                if Path::new("/proc/").join(p).exists() {
                    return Ok(dbg!(Status::RunFail(
                        "PROGRAM IS ALREADY RUNNING".to_string()
                    )));
                }
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
                .arg("while [ 1 ] ; do cat input_fifo ; done | java -jar paper.jar --nogui > server_output")
                .spawn()
                .expect("failed to execute process");

            pid = command.id();

            let _file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(pid_file)
                .unwrap()
                .write_all(pid.to_string().as_bytes());

            Ok(Status::RunOk)
        }

        pub async fn stop_self(&mut self) -> Result<Status> {
            let working_dir = Path::new(&self.working_directory);
            let pid_file = working_dir.join("process_id");
            let p = dbg!(std::fs::read_to_string(&pid_file)).unwrap();

            if !pid_file.exists() || !Path::new("/proc/").join(p.clone()).exists() {
                return Ok(dbg!(Status::StopFail("Program isnt running".to_string())));
            }

            let mut file: std::fs::File = std::fs::OpenOptions::new()
                .write(true)
                .open(Path::new(&self.working_directory).join("input_fifo"))
                .unwrap();

            let _ = writeln!(file, "stop");

            // WE NEED TO RELEASE THE PID FROM THE PROCESS TABLE
            use sysinfo::{Pid, System};

            let s = System::new_all();
            if let Some(process) = s.process(Pid::from(p.parse::<usize>().unwrap())) {
                process.kill_with(Signal::Term);
            }
            Ok(Status::StopOk)
        }

        pub fn new(dir: PathBuf) -> Self {
            let mut properties: Option<String> = None;
            let working_dir = Path::new(&dir);

            dbg!(&dir);

            for file_path in std::fs::read_dir(working_dir).unwrap() {
                let file_path_string = file_path
                    .unwrap()
                    .path()
                    .into_os_string()
                    .into_string()
                    .unwrap_or("".to_string());

                let file = file_path_string.rsplit("/").collect::<Vec<&str>>()[0];

                match file {
                    "server.properties" => properties = Some(file_path_string.to_string()),
                    _ => (),
                }
            }
            let server = Server {
                properties_path: properties,
                working_directory: dir.to_str().unwrap().to_string(),
                ..Default::default()
            };
            server
        }

        pub async fn create_new_server(
            // self
            server_name: &str,
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

            Ok(Server {
                working_directory: server_dir.to_str().unwrap().to_string(),
                ..Default::default()
            })
        }
    }
}
