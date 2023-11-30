use std::i32;

use rocket::FromForm;

#[derive(FromForm)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub published: bool,
}

impl Question {
    pub fn new(id: i32, question: String, published: bool) -> Question {
        Question {
            id,
            question,
            published,
        }
    }
}

pub struct NewQuestion {
    pub question: String,
    pub published: bool,
}

impl NewQuestion {
    pub fn new(question: String, published: Option<bool>) -> NewQuestion {
        NewQuestion {
            question,
            published: published.unwrap_or(false),
        }
    }
}

pub struct QuestionRepository {
    connection: &'static str,
}
