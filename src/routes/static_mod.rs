use rocket::get;
use rocket::response::content::RawCss;

#[get("/static/css/style.css")]
pub async fn style() -> RawCss<&'static str> {
    let a = RawCss(include_str!("../../static/css/style.css"));
    a
}
