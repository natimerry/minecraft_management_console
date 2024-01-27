#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut mc = minecraft_manager::McServerManager::new()
        .set_directory("./mc")
        .set_cache_directory("./cache.txt");
    dbg!(mc.get_available_versions().await.unwrap());
    Ok(())
}
