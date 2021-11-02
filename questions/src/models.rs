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
struct QuestionCreate<'r> {
    body: &'r str,
    user_id: i32
}