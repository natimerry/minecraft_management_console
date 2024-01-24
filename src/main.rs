use rocket::{
    self,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;
use std::env;

mod login;
use login::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("templates")))
        .mount("/", routes![login_page])
        .mount("/", routes![login_auth])
        .mount("/", routes![registered])
}
