use minecraft_manager;
use password_lib::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
struct Token {
    user_name: String,
    token: String,
}

#[allow(private_interfaces)]
#[post("/console", data = "<token>")]
pub fn console_page(token: Form<Token>) -> Template {
    let mut mc_manager = minecraft_manager::McServerManager::new()
        .set_directory("./src/minecraft_manager/mc".to_string());

    let servers = mc_manager.get_installations().unwrap();
    Template::render("console", context! {servers})
}
