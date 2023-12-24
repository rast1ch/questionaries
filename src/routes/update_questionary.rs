use rocket::{form::Form, get, put, response::Redirect, serde::json::Json};
use rocket_dyn_templates::Template;

use crate::{
    db::DbConn, forms::create_questionary::Questionary, models::questionary::QuestionaryRepository,
};

#[get("/questionary/<id>/edit")]
pub async fn edit_questionary(id: i32, connection: &DbConn) -> Result<Template, Redirect> {
    let questionary = QuestionaryRepository::get(id, &connection).await;

    match questionary {
        Ok(questionary) => Result::Ok(Template::render("questionary/edit", questionary)),
        Err(_) => Result::Err(Redirect::to("/404")),
    }
}
#[put("/questionary/<id>/edit", data = "<questionary>")]
pub async fn update_questionary(
    id: i32,
    questionary: Form<Questionary<'_>>,
    connection: &DbConn,
) -> Redirect {
    let questionary =
        QuestionaryRepository::update(id, questionary.into_inner(), &connection).await;
    match questionary {
        Ok(_) => Redirect::to(format!("/questionary/{}", id)),
        Err(_) => Redirect::to("/"),
    }
}
