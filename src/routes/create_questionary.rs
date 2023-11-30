use rocket::{
    form::Form,
    post,
    serde::json::{self},
    FromForm,
};
use rocket_dyn_templates::Template;

use crate::models::questions::Question;

#[derive(FromForm)]
pub struct Questionary {
    pub name: String,
    pub questions: Question,
}

#[post("/create_questionary", data = "<questionary>")]
pub fn create_questionary(questionary: Form<Questionary>) -> Template {
    let context: json::Value = json::from_str(r#"{"name": "world"}"#).unwrap();
    Template::render("index", &context)
}
