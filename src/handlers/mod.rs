use rocket::{serde::json::Json};

use crate::models::*;
mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    todo!()
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    todo!()
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    todo!()
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    todo!()
}

#[get("/answer", data = "<question_uuid>")]
pub async fn read_answers(question_uuid: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    todo!()
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    todo!()
}