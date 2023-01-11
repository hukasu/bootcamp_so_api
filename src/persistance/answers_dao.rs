use std::borrow::Cow;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::{models::{Answer, AnswerDetail}, errors};

use error_stack::{IntoReport, Result, ResultExt};
use crate::errors::DBError;

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .into_report()
            .change_context(DBError::InvalidUUID(answer.question_uuid))
            .attach_printable_lazy(|| "Failed to parse Question Uuid while creating answer.")?;

        let sql_result = sqlx::query!(
                r#"INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *;"#,
                uuid,
                answer.content
            )
            .fetch_one(&self.db)
            .await;
        let record = match sql_result {
            Ok(r) => Ok(r),
            Err(r) => {
                match r.as_database_error() {
                    Some(dberr) => {
                        match dberr.code() {
                            Some(
                                Cow::Borrowed(
                                    errors::postgres_error_codes::FOREIGN_KEY_VIOLATION
                                )
                            ) => Err(r).into_report().change_context(DBError::InvalidUUID(uuid.to_string())),
                            _ => Err(r).into_report().change_context(DBError::Other)
                        }
                    },
                    _ => Err(r).into_report().change_context(DBError::Other)
                }
                .attach_printable_lazy(|| "Failed to create question on database.")
            }
        }?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .into_report()
            .change_context(DBError::InvalidUUID(answer_uuid))
            .attach_printable_lazy(|| "Failed to parse Answer Uuid while deleting answer.")?;

        sqlx::query!(
                r#"DELETE FROM answers WHERE answer_uuid = $1;"#,
                uuid
            )
            .execute(&self.db)
            .await
            .into_report()
            .change_context(DBError::Other)
            .attach_printable_lazy(|| "Failed to delete answer from database.")?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .into_report()
            .change_context(DBError::InvalidUUID(question_uuid))
            .attach_printable_lazy(|| "Failed to parse Question Uuid while recovering answers.")?;

        // Make a database query to get all answers associated with a question uuid.
        // Here is the SQL query:
        // ```
        // SELECT * FROM answers WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = sqlx::query!(
                r#"SELECT * FROM answers WHERE question_uuid = $1;"#,
                uuid
            )
            .fetch_all(&self.db)
            .await
            .into_report()
            .change_context(DBError::Other)
            .attach_printable_lazy(|| "Failed to delete answer from database.")?;

        // Iterate over `records` and map each record to a `AnswerDetail` type
        let answers = records.into_iter()
            .map(
                |r| {
                    AnswerDetail {
                        answer_uuid: r.answer_uuid.to_string(),
                        question_uuid: r.question_uuid.to_string(),
                        content: r.content,
                        created_at: r.created_at.to_string()
                    }
                }
            )
            .collect();

        Ok(answers)
    }
}
