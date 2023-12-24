use rocket::serde::{Deserialize, Serialize};

use crate::db::DbConn;
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserAnswer {
    pub question_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserAnswerList {
    user_answers: Vec<UserAnswer>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserAnswerStatisticsResponse {
    pub question_id: i32,
    pub answer_id: i32,
    pub answer_text: String,
    pub count: Option<i64>,
}

pub struct UserAnswerRepository {}

impl UserAnswerRepository {
    pub async fn create_user_answer(user_answer: UserAnswer, pool: &DbConn) -> Result<(), ()> {
        sqlx::query!(
            "INSERT INTO user_answer ( answer_id) VALUES ($1)",
            user_answer.answer_id
        )
        .execute(&pool.0)
        .await;
        Ok(())
    }
    pub async fn get_user_answer_statistics(
        question_id: i32,
        pool: &DbConn,
    ) -> Result<Vec<UserAnswerStatisticsResponse>, sqlx::Error> {
        sqlx::query_as!(
            UserAnswerStatisticsResponse,
            "SELECT question_id, answer_id, answer.answer answer_text, count(*) as count FROM user_answer INNER JOIN answer ON user_answer.answer_id = answer.id INNER JOIN question ON answer.question_id = question.id WHERE question.id = $1 GROUP BY question_id, answer_id, answer.answer",
            question_id
        )
        .fetch_all(&pool.0)
        .await
    }
}
