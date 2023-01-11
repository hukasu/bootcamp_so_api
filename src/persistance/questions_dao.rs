use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{Question, QuestionDetail};

use error_stack::{IntoReport, Result, ResultExt};
use crate::errors::DBError;

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(r#"
INSERT INTO questions ( title, description )
VALUES ( $1, $2 )
RETURNING *;"#,
                question.title,
                question.description
            )
            .fetch_one(&self.db)
            .await
            .into_report()
            .change_context(DBError::Other)
            .attach_printable_lazy(|| "Failed to create question on database.")?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .into_report()
            .change_context(DBError::InvalidUUID(question_uuid))
            .attach_printable_lazy(|| "Failed to parse Uuid while deleting question.")?;

        sqlx::query!(
                r#"DELETE FROM questions WHERE question_uuid = $1;"#,
                uuid
            )
            .execute(&self.db)
            .await
            .into_report()
            .change_context(DBError::Other)
            .attach_printable_lazy(|| "Failed to delete question from database.")?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!(
                r#"SELECT * FROM questions;"#
            )
            .fetch_all(&self.db)
            .await
            .into_report()
            .change_context(DBError::Other)
            .attach_printable_lazy(|| "Failed to recover question into database.")?;

        let questions = records.into_iter()
            .map(
                |r| {
                    QuestionDetail {
                        question_uuid: r.question_uuid.to_string(),
                        title: r.title,
                        description: r.description,
                        created_at: r.created_at.to_string()
                    }
                }
            )
            .collect();

        Ok(questions)
    }
}
