use crate::db::DbConn;
use crate::models::questionary::QuestionaryRepository;
use rocket::get;
use rocket_dyn_templates::Template;

#[get("/list_questionaries")]
pub async fn list_questionaries(pool: &DbConn) -> Template {
    let questionaries = QuestionaryRepository::get_all(pool).await.unwrap();
    Template::render("list_questionaries", questionaries)
}
