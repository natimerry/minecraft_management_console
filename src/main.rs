use rocket::{
    self,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;
use std::env;

mod login;
mod console;
use login::*;
use console::{console_page, tx_channel};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("templates")))
        .mount("/", routes![login_page,login_auth,registered]) // auth
        .mount("/", routes![console_page,tx_channel]) // console
}
