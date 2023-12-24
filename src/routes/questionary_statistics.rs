use rocket::get;
use rocket_dyn_templates::Template;

use crate::{
    db::DbConn,
    models::questionary::{QuestionaryRepository, QuestionaryUserAnswerStatisticsResponse},
};

#[get("/questionary/<id>/statistics")]
pub async fn get_questionary_statistics(id: i32, pool: &DbConn) -> Template {
    let questionary = QuestionaryRepository::get(id, &pool).await.unwrap();
    let questionary = QuestionaryUserAnswerStatisticsResponse::from_sql(questionary, &pool).await;
    Template::render("questionary_statistics", questionary)
}
