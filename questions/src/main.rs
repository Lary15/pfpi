#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod db;

use models::*;

// use self::diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize)]
struct Answer {
    id: i32,
    body: String,
    user_id: i32,
    question_id: i32
}

#[derive(Deserialize)]
struct AnswerCreate<'r> {
    body: &'r str,
    user_id: i32,
    question_id: i32
}

#[get("/question", format = "json")]
fn get_questions() -> Json<Vec<Question>> {
    let list:Vec<Question> = db::get_questions();
    Json(list)
}

#[post("/question", format = "json", data= "<question>")]
fn create_question(question: Json<QuestionCreate<'_>>)  -> Json<Question> {
    Json( db::create_question(&question))
}

#[get("/question/<question_id>")]
fn get_question_id(question_id: i32) -> Json<Option<Question>> {
    let q = db::find_by_id(question_id);

    if let Some(ques) = q  {
        return Json(Some(ques));
    }
    return Json(None);
}

#[get("/answer/<question_id>")]
fn get_answers(question_id: i32) -> Json<Vec<Answer>> {
    let mut list:Vec<Answer> = vec![];
    list.push(Answer{id: 1, body: String::from("O que esta acontencendo?"), user_id: 1, question_id: 1});
    Json(list)
}

#[post("/answer", format = "json", data= "<answer>")]
fn create_answer(answer: Json<AnswerCreate<'_>>)  -> Json<Answer> {
    Json(Answer {
        id: 1,
        body: answer.body.to_owned(),
        user_id: answer.user_id,
        question_id: answer.question_id
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_answers, create_answer])
        .mount("/", routes![get_questions, create_question])
        .mount("/", routes![get_question_id])
}
