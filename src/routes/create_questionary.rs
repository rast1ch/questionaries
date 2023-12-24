use crate::models::questionary::QuestionaryRepository;
use crate::{db::DbConn, forms::create_questionary::Questionary};
use rocket::response::Redirect;
use rocket::{form::Form, get, post};
use rocket_dyn_templates::Template;

#[get("/create_questionary")]
pub fn create_questionary_form() -> Template {
    Template::render("create_questionary", ())
}
#[post("/create_questionary", data = "<questionary>")]
pub async fn create_questionary(pool: &DbConn, questionary: Form<Questionary<'_>>) -> Redirect {
    let questionary = questionary.into_inner();
    QuestionaryRepository::create(questionary, pool)
        .await
        .unwrap();
    Redirect::to("/list_questionaries")
}
