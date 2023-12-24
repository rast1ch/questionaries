use std::collections::HashMap;

use rocket::{
    form::Form,
    get, post,
    serde::json::{self, Json},
    FromForm,
};
use rocket_dyn_templates::{context, Template};

use crate::{
    db::DbConn,
    models::user_answer::{UserAnswer, UserAnswerRepository},
};

#[derive(FromForm)]
pub struct QuestionarySubmit<'r> {
    json: &'r str,
}

#[post("/questionary/<id>", data = "<questionary>")]
pub async fn submit_questionary(
    id: i32,
    questionary: Form<QuestionarySubmit<'_>>,
    pool: &DbConn,
) -> Result<Template, ()> {
    println!("submit_questionary");
    println!("id: {}", id);
    let ques: serde_json::Value =
        serde_json::from_str::<Option<serde_json::Value>>(questionary.into_inner().json)
            .unwrap()
            .unwrap();
    let mut map = HashMap::new();
    for (key, value) in ques.as_object().unwrap() {
        map.insert(key, value.as_str().unwrap());
    }
    for (key, value) in map.iter() {
        UserAnswerRepository::create_user_answer(
            UserAnswer {
                question_id: key.parse::<i32>().unwrap(),
                answer_id: value.parse::<i32>().unwrap(),
            },
            pool,
        )
        .await?;
    }
    Ok(Template::render("questionary", context! {}))
}

#[get("/questionary/<id>/ty_page")]
pub async fn ty_page(id: i32, pool: &DbConn) -> Template {
    println!("ty_page");
    println!("id: {}", id);
    Template::render("ty_page", context! {})
}
