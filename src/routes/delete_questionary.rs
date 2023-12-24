use rocket::{get, response::Redirect};

use crate::{
    db::DbConn,
    models::questionary::{QuestionaryRepository, QuestionarySQL},
};

#[get("/delete_questionary/<id>")]
pub async fn delete_questionary(id: i32, pool: &DbConn) -> Redirect {
    let questionary: Result<QuestionarySQL, _> = QuestionaryRepository::get(id, pool).await;
    match questionary {
        Ok(_) => {
            QuestionaryRepository::delete(id, pool).await.unwrap();
            Redirect::to("/list_questionaries")
        }
        Err(_) => Redirect::to("/list_questionaries"),
    }
}
