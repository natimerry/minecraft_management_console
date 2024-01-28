use minecraft_manager::{self};
use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::{self, form::FromForm, get, post};
use rocket_dyn_templates::{context, Template};
