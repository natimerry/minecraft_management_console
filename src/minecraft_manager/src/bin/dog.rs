

use minecraft_manager::server::mc_server::Server;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut server = Server{
        ..Default::default()
    };

    
    Ok(())
}
