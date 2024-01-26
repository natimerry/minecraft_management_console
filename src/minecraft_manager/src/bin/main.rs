#[tokio::main]
async fn main() -> Result<(),()>{
    let mut mc = minecraft_manager::McServerManager::new().set_directory("./mc".to_string());
    dbg!(mc.get_available_versions().await);
    Ok(())
}
