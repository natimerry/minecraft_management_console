use minecraft_manager::server::{mc_server::Server};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut mc_manager = minecraft_manager::McServerManager::new(Some("../../mc_server_manager.json"));

    mc_manager.stop_server("pvp_server").await;
    Ok(())
}
