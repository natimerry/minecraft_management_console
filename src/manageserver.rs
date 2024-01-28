use minecraft_manager::{self};
use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};

#[get("/status/<name>", rank = 1)]
pub fn ws_server_status<'a>(ws: ws::WebSocket, name: &'a str) -> ws::Channel<'a> {
    let ws = ws.config(ws::Config {
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mc_manager = minecraft_manager::McServerManager::new(None);
            let server_status = mc_manager.get_server_status(name);
            let x = match server_status{
                true => "Active",
                false => "Inactive",
            };

            let status = stream.send(x.into()).await;

            Ok(())
        })
    })
}

#[get("/start/<name>", rank = 1)]
pub fn ws_server_start<'a>(ws: ws::WebSocket, name: &'a str) -> ws::Channel<'a> {
    let ws = ws.config(ws::Config {
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mc_manager = minecraft_manager::McServerManager::new(None);
            mc_manager.run_server(name).await;            
            Ok(())
        })
    })
}

#[get("/stop/<name>", rank = 1)]
pub fn ws_server_stop<'a>(ws: ws::WebSocket, name: &'a str) -> ws::Channel<'a> {
    let ws = ws.config(ws::Config {
        max_message_size: Some(5 << 20),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mc_manager = minecraft_manager::McServerManager::new(None);
            mc_manager.stop_server(name).await;            
            Ok(())
        })
    })
}