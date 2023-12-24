use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn main() -> Template {
    let context = context!();
    Template::render("index", &context)
}
