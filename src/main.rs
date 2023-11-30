use rocket::{
    serde::json::{self},
    *,
};
pub mod models;
pub mod routes;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/1")]
fn index2() -> Template {
    let context: json::Value = json::from_str(r#"{"name": "world"}"#).unwrap();
    Template::render("index", &context)
}

#[launch]
async fn rocket() -> _ {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://questions:questions@db:5432/questions")
    //     .await;
    // let res_ = sqlx::query!(
    //     "INSERT INTO questionaire (title, description) VALUES ($1, $2)",
    //     "What is your name?",
    //     "New questions"
    // )
    // .execute(&pool.unwrap())
    // .await
    // .unwrap();
    // println!("res_: {:?}", res_);
    rocket::build()
        .mount(
            "/hello",
            routes![
                index,
                index2,
                routes::create_questionary::create_questionary
            ],
        )
        .attach(Template::fairing())
}
