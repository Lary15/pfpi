use super::schema::questions;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Clone)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "questions"]
pub struct QuestionCreate<'r> {
    pub body: &'r str,
    pub user_id: i32
}


#[derive(Queryable, Serialize, Clone)]
struct Answer {
    id: i32,
    body: String,
    user_id: i32,
    question_id: i32
}

#[derive(Deserialize, Insertable)]
#[table_name = "answers"]
struct AnswerCreate<'r> {
    body: &'r str,
    user_id: i32,
    question_id: i32
}
