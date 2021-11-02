use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::*;

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", 
                                   database_url))
}

pub fn create_question(question: &QuestionCreate)-> Question {
    use crate::schema::questions;
    use crate::schema::questions::dsl::*;

    let connection = establish_connection();

    diesel::insert_into(questions::table)
        .values(question)
        .execute(&connection)
        .expect("Error saving new post");
    
    let id_ = diesel::select(last_insert_rowid)
        .get_result::<i32>(&connection).expect(""); // <--- returns Result<i32, Error>

    question_find_by_id(id_).unwrap()
}

pub fn get_questions() -> Vec<Question> {
    use crate::schema::questions;
    use crate::schema::questions::dsl::*;

    let connection = establish_connection();
    questions
        .load::<Question>(&connection)
        .expect("Error loading posts")
}

pub fn question_find_by_id(id_: i32) -> Option<Question>{
    use crate::schema::questions;
    use crate::schema::questions::dsl::*;
    let connection = establish_connection();

    let result = questions
        .filter(id.eq(id_))
        .load::<Question>(&connection)
        .expect("Error loading posts");

    if result.len() > 0{
        return Some(result[0].clone());
    }
    else{ return None;}
}

pub fn answer_find_by_question_id(id_: i32) -> Vec<Answer>{
    use crate::schema::answers;
    use crate::schema::answers::dsl::*;
    let connection = establish_connection();

    answers
        .filter(question_id.eq(id_))
        .load::<Answer>(&connection)
        .expect("Error loading posts")
}

pub fn answer_find_by_id(id_: i32) -> Option<Answer>{
    use crate::schema::answers;
    use crate::schema::answers::dsl::*;
    let connection = establish_connection();

    let result = answers
        .filter(id.eq(id_))
        .load::<Answer>(&connection)
        .expect("Error loading posts");

    if result.len() > 0{
        return Some(result[0].clone());
    }
    else{ return None;}
}


pub fn create_answer(question: &AnswerCreate)-> Answer {
    use crate::schema::answers;
    use crate::schema::answers::dsl::*;
    let connection = establish_connection();

    diesel::insert_into(answers::table)
        .values(question)
        .execute(&connection)
        .expect("Error saving new post");
    
    let id_ = diesel::select(last_insert_rowid)
        .get_result::<i32>(&connection).expect(""); // <--- returns Result<i32, Error>

    answer_find_by_id(id_).unwrap()
}

pub fn get_answers() -> Vec<Answer> {
    use crate::schema::answers;
    use crate::schema::answers::dsl::*;

    let connection = establish_connection();
    answers
        .load::<Answer>(&connection)
        .expect("Error loading posts")
}
