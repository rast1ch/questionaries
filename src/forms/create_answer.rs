use rocket::FromForm;

#[derive(FromForm)]
pub struct Answer<'r> {
    pub id: i32,
    pub question_id: i32,
    pub answer: &'r str,
}
