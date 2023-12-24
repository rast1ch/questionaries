use crate::{db::DbConn, forms::create_answer::Answer};
use rocket::serde::{Deserialize, Serialize};

pub struct AnswerSQL {
    pub id: i32,
    pub answer: String,
    pub question_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AnswerResponse {
    pub id: i32,
    pub answer: String,
    pub question_id: i32,
}

impl AnswerResponse {
    pub fn from(answer: AnswerSQL) -> Self {
        AnswerResponse {
            id: answer.id,
            answer: answer.answer,
            question_id: answer.question_id,
        }
    }
}

pub struct AnswerRepository {}
impl AnswerRepository {
    pub async fn get(id: i32, pool: &DbConn) -> Result<AnswerSQL, sqlx::Error> {
        sqlx::query_as!(
            AnswerSQL,
            "SELECT id, answer, question_id FROM answer WHERE id = $1",
            id
        )
        .fetch_one(&pool.0)
        .await
    }
    pub async fn get_by_question(
        question_id: i32,
        pool: &DbConn,
    ) -> Result<Vec<AnswerSQL>, sqlx::Error> {
        sqlx::query_as!(
            AnswerSQL,
            "SELECT id, answer, question_id FROM answer WHERE question_id = $1",
            question_id
        )
        .fetch_all(&pool.0)
        .await
    }
    pub async fn list(pool: &DbConn) -> Result<Vec<AnswerSQL>, sqlx::Error> {
        sqlx::query_as!(AnswerSQL, "SELECT id, answer, question_id FROM answer")
            .fetch_all(&pool.0)
            .await
    }
    pub async fn create(answer: Answer<'_>, pool: &DbConn) -> Result<AnswerSQL, sqlx::Error> {
        sqlx::query_as!(
            AnswerSQL,
            "INSERT INTO answer (id, answer, question_id) VALUES ($1, $2, $3) RETURNING id, answer, question_id",
            answer.id,
            answer.answer,
            answer.question_id
        )
        .fetch_one(&pool.0)
        .await
    }
}
