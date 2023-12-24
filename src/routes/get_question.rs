use rocket::{get, response::Redirect};
use rocket_dyn_templates::Template;

use crate::{
    db::DbConn,
    models::{
        questionary::{QuestionaryRepository, QuestionaryResponse},
        questions::QuestionResponse,
    },
};

#[get("/questionary/<id>")]
pub async fn get_questionary(id: i32, connection: &DbConn) -> Result<Template, Redirect> {
    let questionary_obj = QuestionaryRepository::get(id, &connection).await;
    let questionary = match questionary_obj {
        Ok(questionary) => QuestionaryResponse::from_sql(questionary, &connection).await,
        Err(_) => return Result::Err(Redirect::to("/404")),
    };

    Result::Ok(Template::render("get_questionary", questionary))
}
