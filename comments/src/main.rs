#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;


pub mod models;
pub mod schema;
pub mod connection;
pub mod request;
pub mod responses;

use rocket::serde::{Deserialize, Serialize, json::Json};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::*;
use self::connection::connection_db;
use self::request::*;
use self::responses::*;




#[post("/create_answer?<user_id>&<answer_id>", data = "<comment>")]
pub fn create_comment_answer<'a>(user_id: i32, answer_id: i32, comment: Json<Comment<'_>>) -> Json<ResponseComment> {

    use schema::comments;

    let conn = connection_db();

    let new_comment = NewComment {
        answer_id: Some(answer_id),
        question_id: None,
        user_id: user_id, 
        comment: comment.message,
    };

    let comment : LoadComment;

    comment = diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result(&conn)
        .expect("Error saving new comment for answer!!");

    let qa_id = comment.answer_id.unwrap();

    return Json(ResponseComment { user_id: comment.user_id, qa_id: qa_id, comment: comment.comment, id: comment.id  })
}

#[post("/create_question?<user_id>&<question_id>", data = "<comment>")]
fn create_comment_question(user_id: i32, question_id: i32, comment: Json<Comment<'_>> ) -> Json<ResponseComment> {

  use schema::comments;

  let conn = connection_db();

  let new_comment = NewComment {
      answer_id: None,
      question_id: Some(question_id),
      user_id: user_id, 
      comment: comment.message,
  };

  let comment : LoadComment;

  comment = diesel::insert_into(comments::table)
      .values(&new_comment)
      .get_result(&conn)
      .expect("Error saving new comment for answer!!");

  let question_id = comment.question_id.unwrap();

  return Json(ResponseComment { user_id: comment.user_id, qa_id: question_id, comment: comment.comment, id: comment.id  })
}

#[put("/edit",data = "<comment>")]
fn edit_comment(comment: Json<EditComment<'_>>) -> Json<EditResponse> {
    return Json(EditResponse { message: "Comentário Editado com sucesso"});
}

#[delete("/delete?<comment_id>")]
fn delete_comment(comment_id: i32) -> Json<DeleteResponse> {
    return Json(DeleteResponse { message: "Comentário Deletado com sucesso" });
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/comment", routes![create_comment_answer, create_comment_question, edit_comment, delete_comment])
}
