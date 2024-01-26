use minecraft_manager;
use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
struct Token {
    user_name: String,
    token: String,
}

#[allow(private_interfaces)]
#[post("/console", data = "<token>")]
pub async fn console_page(token: Form<Token>) -> Template {
    let mut mc_manager = minecraft_manager::McServerManager::new()
        .set_directory("./src/minecraft_manager/mc");

    let servers = mc_manager.get_installations().unwrap();

    Template::render("console", context! {servers,})
}

#[get("/rx", rank = 1)]
pub fn tx_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    let ws = ws.config(ws::Config {
        // set max message size to 3MiB
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut versions = minecraft_manager::McServerManager::new()
                .set_cache_directory("./cache.txt")
                .get_available_versions()
                .await
                .unwrap()
                .keys()
                .map(|key| {
                    println!("{}",key);
                    key.clone()
                })
                .collect::<Vec<String>>();
                
                versions.sort_by(|k,v|{
                    let v1 = &k[2..4].replace(".", "");
                    let v2 = &v[2..4].replace(".", "");

                    v1.parse::<i32>().unwrap().cmp(&v2.parse::<i32>().unwrap())                    
                    // dbg!(&k[2..4].replace(".", "")).parse::<i32>().unwrap().cmp(&v[2..4].replace(".", to).parse::<i32>().unwrap())
                });
                versions.reverse();
            // 
            for i in versions{
                println!("SENDING VERSION: {}",i);
                stream.send(i.clone().into()).await.unwrap();
            }
            Ok(())
        })
    })
}
