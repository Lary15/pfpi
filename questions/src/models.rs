use super::schema::questions;
use super::schema::answers;
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
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
    pub question_id: i32
}

#[derive(Deserialize, Insertable)]
#[table_name = "answers"]
pub struct AnswerCreate<'r> {
    pub body: &'r str,
    pub user_id: i32,
    pub question_id: i32
}
