#![feature(lazy_cell)]
#![feature(async_closure)]
#![feature(sync_unsafe_cell)]
use rocket::{
    self,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;
use std::env;

mod login;
mod console;
mod manageserver;

use manageserver::*;
use login::*;
use console::{console_page, create_new_server, tx_channel,*};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("templates")))
        .mount("/", routes![login_page,login_auth,registered]) // auth
        .mount("/", routes![console_page,tx_channel,create_new_server,ws_channel_create]) // console
        .mount("/", routes![ws_server_status,ws_server_start,ws_server_stop])
}
