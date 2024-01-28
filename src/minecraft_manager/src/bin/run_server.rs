use minecraft_manager::server::{mc_server::Server};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut mc_manager = minecraft_manager::McServerManager::new(Some("../../mc_server_manager.json"));
    // mc_manager.create_new_server("1.20.4", "pvp_server").await;
    // let _ = server.run_self("./mc/test_server").await;

    mc_manager.run_server("pvp_server").await;

    Ok(())
}
