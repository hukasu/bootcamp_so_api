#[macro_use]
extern crate rocket;

mod cors;
mod handlers;
mod models;
mod persistance;
mod errors;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenvy::dotenv().unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
