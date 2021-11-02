use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::*;


fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", 
                                   database_url))
}

pub fn create_question(question: &QuestionCreate) {
    use crate::schema::questions;
    use crate::schema::questions::dsl::*;

    let connection = establish_connection();

    diesel::insert_into(questions::table)
        .values(question)
        .execute(&connection)
        .expect("Error saving new post");
}

pub fn get_questions() -> Vec<Question> {
    use crate::schema::questions;
    use crate::schema::questions::dsl::*;

    let connection = establish_connection();
    questions
        .load::<Question>(&connection)
        .expect("Error loading posts")
}