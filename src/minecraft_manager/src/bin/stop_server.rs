use minecraft_manager::server::{mc_server::Server};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut _server = Server {
        working_directory: "./mc/pvp_server".to_string(),
        ..Default::default()
    };



    dbg!(_server.stop_self().await);

    Ok(())
}
