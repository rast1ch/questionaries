use rocket::FromForm;

#[derive(FromForm)]
pub struct Questionary<'r> {
    pub title: &'r str,
    pub description: &'r str,
}
