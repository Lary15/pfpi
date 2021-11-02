use super::schema::questions;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
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