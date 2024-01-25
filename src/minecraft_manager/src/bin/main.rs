fn main(){
    dbg!(minecraft_manager::McServerManager::new().set_directory("./mc".to_string()).get_installations());
}