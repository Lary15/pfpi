#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;


pub mod models;
pub mod schema;
pub mod connection;
pub mod request;
pub mod responses;

use rocket::serde::json::Json;
use diesel::prelude::*;
use self::models::*;
use self::connection::connection_db;
use self::request::*;
use self::responses::*;




#[post("/create_answer?<user_id>&<answer_id>", data = "<comment>")]
pub fn create_comment_answer<'a>(user_id: i32, answer_id: i32, comment: Json<Comment<'_>>) -> Json<ResponseComment> {

    use schema::comments;

    let conn = connection_db();

    let new_comment = NewComment {
        answer_id: answer_id,
        question_id: 0,
        user_id: user_id, 
        comment: comment.message,
    };

    let comment : LoadComment;

    comment = diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result(&conn)
        .expect("Error saving new comment for answer!!");

    let qa_id = comment.answer_id;

    return Json(ResponseComment { user_id: comment.user_id, qa_id: qa_id, comment: comment.comment, id: comment.id  })
}

#[post("/create_question?<user_id>&<question_id>", data = "<comment>")]
fn create_comment_question(user_id: i32, question_id: i32, comment: Json<Comment<'_>> ) -> Json<ResponseComment> {

  use schema::comments;

  let conn = connection_db();

  let new_comment = NewComment {
      answer_id: 0,
      question_id: question_id,
      user_id: user_id, 
      comment: comment.message,
  };

  let comment : LoadComment;

  comment = diesel::insert_into(comments::table)
      .values(&new_comment)
      .get_result(&conn)
      .expect("Error saving new comment for answer!!");

  let question_id = comment.question_id;

  return Json(ResponseComment { user_id: comment.user_id, qa_id: question_id, comment: comment.comment, id: comment.id  })
}

#[put("/edit",data = "<comment_request>")]
fn edit_comment(comment_request: Json<EditComment<'_>>) -> Json<EditResponse> {

    use schema::comments::dsl::{comment, comments};

    let conn = connection_db();


    diesel::update(comments.find(comment_request.id))
        .set(comment.eq(comment_request.message))
        .execute(&conn)
        .expect("Error updating comment");

    return Json(EditResponse { message: "Comentário Editado com sucesso"});
}

#[delete("/delete?<comment_id>")]
fn delete_comment(comment_id: i32) -> Json<DeleteResponse> {

    use schema::comments::dsl::*;

    let conn = connection_db();

    diesel::delete(comments.find(comment_id))
        .execute(&conn)
        .expect("Error deleting comment");

    return Json(DeleteResponse { message: "Comentário Deletado com sucesso" });
}

#[get("/get_all_question?<id_question>")]
fn get_all_comment_question(id_question: i32) -> Json<GetAllResponse> {

    use schema::comments::dsl::*;

    let conn = connection_db();

    let result = comments.filter(question_id.eq(id_question))
        .load::<LoadComment>(&conn)
        .expect("Error loading comments");
    
    
    return Json(GetAllResponse { comments: result });
}

#[get("/get_all_comment_answer?<id_answer>")]
fn get_all_comment_answer(id_answer: i32) -> Json<GetAllResponse> {
    
    use schema::comments::dsl::*;

    let conn = connection_db();

    let result = comments.filter(answer_id.eq(id_answer))
        .load::<LoadComment>(&conn)
        .expect("Error loading comments");

    return Json(GetAllResponse { comments: result});
}


#[get("/get_all")]
fn get_all() -> Json<GetAllResponse> {
    
    use schema::comments::dsl::*;

    let conn = connection_db();

    let result = comments
        .load::<LoadComment>(&conn)
        .expect("Error loading comments");

    return Json(GetAllResponse { comments: result});
}






#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/comment", routes![create_comment_answer, create_comment_question, edit_comment, delete_comment, get_all_comment_question, get_all_comment_answer, get_all])
}
