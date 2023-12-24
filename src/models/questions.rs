use std::i32;

use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;

use crate::db::DbConn;

use super::answer::{AnswerRepository, AnswerResponse};
use super::user_answer::{UserAnswerRepository, UserAnswerStatisticsResponse};

#[derive(FromForm)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub questionaire_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct QuestionSQL {
    id: i32,
    question: String,
    questionaire_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestionResponse {
    pub id: i32,
    pub question: String,
    pub questionaire_id: i32,
    pub anwers: Vec<AnswerResponse>,
}
impl QuestionResponse {
    pub async fn vec_from(questions: Vec<QuestionSQL>, pool: &DbConn) -> Vec<Self> {
        let mut question_response: Vec<QuestionResponse> = Vec::new();
        for question in questions {
            question_response.push(QuestionResponse {
                id: question.id,
                question: question.question,
                questionaire_id: question.questionaire_id,
                anwers: {
                    let answers = AnswerRepository::get_by_question(question.id, &pool).await;
                    match answers {
                        Ok(ansers) => ansers
                            .into_iter()
                            .map(|answer| AnswerResponse::from(answer))
                            .collect(),
                        Err(_) => Vec::new(),
                    }
                },
            });
        }
        question_response
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestionUserAnswerStatisticsResponse {
    pub id: i32,
    pub questionaire_id: i32,
    pub question: String,
    pub answers: Vec<UserAnswerStatisticsResponse>,
    pub sum: i64,
}

impl QuestionUserAnswerStatisticsResponse {
    pub async fn new_vec(id: i32, pool: &DbConn) -> Vec<Self> {
        let mut questions = Vec::new();
        let questions_sql = QuestionRepository::get_by_questionaire(id, &pool)
            .await
            .unwrap();
        for question in questions_sql {
            questions.push(QuestionUserAnswerStatisticsResponse {
                id: question.id,
                questionaire_id: question.questionaire_id,
                question: question.question,
                answers: UserAnswerRepository::get_user_answer_statistics(question.id, &pool)
                    .await
                    .unwrap(),
                sum: UserAnswerRepository::get_user_answer_statistics(question.id, &pool)
                    .await
                    .unwrap()
                    .iter()
                    .map(|answer| answer.count.unwrap_or(0))
                    .sum::<i64>(),
            });
        }
        questions
    }
}

pub struct QuestionRepository {}
impl QuestionRepository {
    pub async fn get(id: i32, pool: &DbConn) -> Result<QuestionSQL, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "SELECT id, question, questionaire_id FROM question WHERE id = $1",
            id
        )
        .fetch_one(&pool.0)
        .await
    }
    pub async fn get_by_questionaire(
        questionaire_id: i32,
        pool: &DbConn,
    ) -> Result<Vec<QuestionSQL>, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "SELECT id, question, questionaire_id FROM question WHERE questionaire_id = $1",
            questionaire_id
        )
        .fetch_all(&pool.0)
        .await
    }
    pub async fn list(pool: &DbConn) -> Result<Vec<QuestionSQL>, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "SELECT id, question, questionaire_id FROM question"
        )
        .fetch_all(&pool.0)
        .await
    }
    pub async fn create(question: Question, pool: &DbConn) -> Result<QuestionSQL, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "INSERT INTO question (id, question, questionaire_id) VALUES ($1, $2, $3) RETURNING id, question, questionaire_id",
            question.id,
            question.question,
            question.questionaire_id
        )
        .fetch_one(&pool.0)
        .await
    }
    pub async fn update(
        id: i32,
        question: Question,
        pool: &DbConn,
    ) -> Result<QuestionSQL, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "UPDATE question SET id = $1, question = $2, questionaire_id = $3 WHERE id = $4 RETURNING id, question, questionaire_id",
            question.id,
            question.question,
            question.questionaire_id,
            id
        )
        .fetch_one(&pool.0)
        .await
    }
    pub async fn delete(id: i32, pool: &DbConn) -> Result<QuestionSQL, sqlx::Error> {
        sqlx::query_as!(
            QuestionSQL,
            "DELETE FROM question WHERE id = $1 RETURNING id, question, questionaire_id",
            id
        )
        .fetch_one(&pool.0)
        .await
    }
}
