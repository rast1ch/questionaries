use questions;
use rocket::{self, launch, routes};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[launch]
pub async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                questions::routes::static_mod::style,
                questions::routes::index::main,
                questions::routes::create_questionary::create_questionary,
                questions::routes::list_questionaries::list_questionaries,
                questions::routes::update_questionary::update_questionary,
                questions::routes::update_questionary::edit_questionary,
                questions::routes::delete_questionary::delete_questionary,
                questions::routes::get_question::get_questionary,
                questions::routes::submit_questionary::submit_questionary,
                questions::routes::submit_questionary::ty_page,
                // questions::routes::get_questionary::get_questionary,
                questions::routes::questionary_statistics::get_questionary_statistics,
            ],
        )
        .attach(Template::fairing())
        .attach(questions::db::DbConn::init())
}
