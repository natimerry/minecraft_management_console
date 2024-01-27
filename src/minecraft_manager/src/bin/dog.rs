

use minecraft_manager::server::mc_server::Server;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut server = Server{
        ..Default::default()
    };

    server.create_new_server("retarddog", "./mc","https://api.papermc.io/v2/projects/paper/versions/1.20.2/builds/318/downloads/paper-1.20.2-318.jar").await;    
    Ok(())
}
