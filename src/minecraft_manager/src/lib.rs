mod server;
use server::Server;

pub struct McServerManager{
    directory: Option<String>,
    installations:Vec<Server>,
    plugins: Vec<String>
}

impl McServerManager{
    pub fn new() -> Self{
        Self {
            directory: None,
            installations: vec![],
            plugins: vec![],
        }
    }
    pub fn set_directory(mut self,directory: String) -> Self{
        self.directory=Some(directory);
        self
    }


}

#[cfg(test)]
mod tests {
    use super::*;
}
