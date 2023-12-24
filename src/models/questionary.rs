use crate::db::DbConn;
use crate::forms::create_questionary::Questionary;
use rocket;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx;

use super::questions::{
    QuestionRepository, QuestionResponse, QuestionUserAnswerStatisticsResponse,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestionaryList {
    questionaries: Vec<QuestionarySQL>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct QuestionarySQL {
    id: i32,
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestionaryResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub questions: Vec<QuestionResponse>,
}
impl QuestionaryResponse {
    pub async fn from_sql(questionary: QuestionarySQL, pool: &DbConn) -> Self {
        QuestionaryResponse {
            id: questionary.id,
            title: questionary.title,
            description: questionary.description,
            questions: QuestionResponse::vec_from(
                QuestionRepository::get_by_questionaire(questionary.id, &pool)
                    .await
                    .unwrap(),
                &pool,
            )
            .await,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuestionaryUserAnswerStatisticsResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub questions: Vec<QuestionUserAnswerStatisticsResponse>,
}

impl QuestionaryUserAnswerStatisticsResponse {
    pub async fn from_sql(questionary: QuestionarySQL, pool: &DbConn) -> Self {
        QuestionaryUserAnswerStatisticsResponse {
            id: questionary.id,
            title: questionary.title,
            description: questionary.description,
            questions: QuestionUserAnswerStatisticsResponse::new_vec(questionary.id, &pool).await,
        }
    }
}

pub struct QuestionaryRepository {}
impl QuestionaryRepository {
    pub async fn create(questionary: Questionary<'_>, pool: &DbConn) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO questionaire (title, description) VALUES ($1, $2)",
            questionary.title,
            questionary.description
        )
        .execute(&pool.0)
        .await?;
        Ok(())
    }
    pub async fn get(id: i32, pool: &DbConn) -> Result<QuestionarySQL, sqlx::Error> {
        let questionary = sqlx::query_as!(
            QuestionarySQL,
            "SELECT id, title, description FROM questionaire WHERE id = $1",
            id
        )
        .fetch_one(&pool.0)
        .await?;
        Ok(questionary)
    }

    pub async fn get_all(pool: &DbConn) -> Result<QuestionaryList, sqlx::Error> {
        let questionaries = sqlx::query_as!(
            QuestionarySQL,
            "SELECT id, title, description FROM questionaire"
        )
        .fetch_all(&pool.0)
        .await;
        match questionaries {
            Ok(questionaries) => Ok(QuestionaryList { questionaries }),
            Err(e) => Err(e),
        }
    }
    pub async fn delete(id: i32, pool: &DbConn) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM questionaire WHERE id = $1", id)
            .execute(&pool.0)
            .await?;
        Ok(())
    }
    pub async fn update(
        id: i32,
        questionary: Questionary<'_>,
        pool: &DbConn,
    ) -> Result<QuestionarySQL, sqlx::Error> {
        let questionary = sqlx::query_as!(
            QuestionarySQL,
            "UPDATE questionaire SET title = $1, description = $2 WHERE id = $3 RETURNING id, title, description",
            questionary.title,
            questionary.description,
            id
        )
        .fetch_one(&pool.0)
        .await?;
        Ok(questionary)
    }
}
