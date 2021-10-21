#[macro_use] extern crate rocket;

use uuid::Uuid;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Deserialize)]
struct Comment <'r> {
    message: & 'r str,
}

#[derive(Deserialize)]
struct EditComment <'r> {
    id: Uuid,
    message: &'r str,
}

#[derive(Deserialize)]
struct GetAllComments {
    id: Uuid,
}

#[derive(Serialize)]
struct ResponseComment <'r> {
    user_id: Uuid,
    qa_id: Uuid,
    id: Uuid,
    comment: & 'r str,
}

#[derive(Serialize)]
struct EditResponse <'r> {
    message: &'r str,
}

#[derive(Serialize)]
struct DeleteResponse {
    message: & 'static str,
}

#[derive(Serialize)]
struct AllComment {
    id: Uuid,
    message: & 'static str,
}

#[derive(Serialize)]
struct GetAllResponse  {
    comments: Vec<AllComment>, 
}




#[post("/create_answer?<user_id>&<answer_id>", data = "<comment>")]
fn create_comment_answer(user_id: Uuid, answer_id: Uuid, comment: Json<Comment<'_>>) -> Json<ResponseComment> {

    let comment_id = Uuid::new_v4();

    return Json(ResponseComment { user_id: user_id, qa_id: answer_id, comment: comment.message, id: comment_id })
}

#[post("/create_question?<user_id>&<question_id>", data = "<comment>")]
fn create_comment_question(user_id: Uuid, question_id: Uuid, comment: Json<Comment<'_>> ) -> Json<ResponseComment> {

    let comment_id = Uuid::new_v4();

    return Json(ResponseComment { user_id: user_id, qa_id: question_id, comment: comment.message, id: comment_id })
}

#[put("/edit",data = "<comment>")]
fn edit_comment(comment: Json<EditComment<'_>>) -> Json<EditResponse> {
    return Json(EditResponse { message: "Comentário Editado com sucesso"});
}

#[delete("/delete?<comment_id>")]
fn delete_comment(comment_id: Uuid) -> Json<DeleteResponse> {
    return Json(DeleteResponse { message: "Comentário Deletado com sucesso" });
}

#[get("/all", data = "<qa>")]
fn get_all_comment(qa: Json<GetAllComments>) -> Json<GetAllResponse> {

    let comment_first = Uuid::new_v4();

    let comment_second  = Uuid::new_v4();

    let comments = vec![AllComment{ id: comment_first, message: "oi lindaaa sz" }, AllComment{ id: comment_second, message: "como você tá ?" }];

    return Json(GetAllResponse { comments: comments });
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/comment", routes![create_comment_answer, create_comment_question, edit_comment, delete_comment, get_all_comment])
}
