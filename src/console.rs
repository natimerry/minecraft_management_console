use minecraft_manager::{self};
use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};


#[derive(FromForm)]
struct Token {
    user_name: String,
    token: String,
}

#[derive(FromForm)]
pub struct NewServer {
    server_name: String,
    version: String,
    user_name: String,
    token: String,
}

#[get("/createserver/<version>/<name>", rank = 1)]
pub fn ws_channel_create(
    ws: ws::WebSocket,
    version: String,
    name: String,
) -> ws::Channel<'static> {
    let ws = ws.config(ws::Config {
        // set max message size to 3MiB
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        let v = version.to_string().clone();
        let name = name.to_string().clone();
        Box::pin(async move {
            let mut mc_manager = minecraft_manager::McServerManager::new(None);
            mc_manager.create_new_server(&v, &name).await;
            let _ = stream.send("DONE".into()).await;
            Ok(())
        })
    })
}
#[allow(private_interfaces)]
#[post("/create", data = "<serverdata>")]
pub async fn create_new_server(serverdata: Form<NewServer>) -> Template {
    Template::render(
        "created",
        context! {
            name:serverdata.server_name.clone(),
        user_name: serverdata.user_name.clone(),
        token: serverdata.token.clone(),
        version: serverdata.version.clone()},
    )
}

#[allow(private_interfaces)]
#[post("/console", data = "<token>")]
pub async fn console_page(token: Form<Token>) -> Template {
    let user_name = token.user_name.clone();
    let token = token.token.clone();
    let mut mc_manager =
        minecraft_manager::McServerManager::new(None);

    let servers = mc_manager.get_installations().unwrap();
    // let names = servers.iter().map(|entry| entry.0.clone()).collect::<Vec<String>>();
    // let versions = servers.iter().map(|entry| entry.0.clone()).collect::<Vec<String>>();

    Template::render("console", context! {servers,user_name,token})
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
            let mut versions = minecraft_manager::McServerManager::new(None)
                .get_available_versions()
                .await
                .unwrap()
                .keys()
                .map(|key| {
                    println!("{}", key);
                    key.clone()
                })
                .collect::<Vec<String>>();

            versions.sort_by(|k, v| {
                let v1 = &k[2..4].replace(".", "");
                let v2 = &v[2..4].replace(".", "");

                v1.parse::<i32>().unwrap().cmp(&v2.parse::<i32>().unwrap())
                // dbg!(&k[2..4].replace(".", "")).parse::<i32>().unwrap().cmp(&v[2..4].replace(".", to).parse::<i32>().unwrap())
            });
            versions.reverse();
            //
            for i in versions {
                // println!("SENDING VERSION: {}",i);
                stream.send(i.clone().into()).await.unwrap();
            }
            Ok(())
        })
    })
}
