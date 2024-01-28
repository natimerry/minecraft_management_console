#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut mc = minecraft_manager::McServerManager::new(None);
    dbg!(mc.get_available_versions().await.unwrap());
    Ok(())
}
