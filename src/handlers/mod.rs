use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{questions_dao::QuestionsDao, answers_dao::AnswersDao},
    errors::APIError
};
mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    handlers_inner::create_question(
            question.0,
            questions_dao
        )
        .await
        .map(|ok| Json::from(ok))
        .map_err(|err| APIError::from(err.current_context().clone()))
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    handlers_inner::read_questions(
            questions_dao
        )
        .await
        .map(|ok| Json::from(ok))
        .map_err(|err| APIError::from(err.current_context().clone()))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>
) -> Result<(), APIError> {
    handlers_inner::delete_question(
            question_uuid.0,
            questions_dao
        )
        .await
        .map_err(|err| APIError::from(err.current_context().clone()))
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>
) -> Result<Json<AnswerDetail>, APIError> {
    handlers_inner::create_answer(
            answer.0,
            answers_dao
        )
        .await
        .map(|ok| Json::from(ok))
        .map_err(|err| APIError::from(err.current_context().clone()))
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    handlers_inner::read_answers(
            question_uuid.0,
            answers_dao
        )
        .await
        .map(|ok| Json::from(ok))
        .map_err(|err| APIError::from(err.current_context().clone()))
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>
) -> Result<(), APIError> {
    handlers_inner::delete_answer(
            answer_uuid.0,
            answers_dao
        )
        .await
        .map_err(|err| APIError::from(err.current_context().clone()))
}