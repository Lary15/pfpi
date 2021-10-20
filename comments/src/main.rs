#[macro_use] extern crate rocket;
// WIP
// use rocket::request::Request;
// use rocket::data::{self,Data, FromData};
// use rocket::http::{Status, ContentType};

// struct CommentAnswer {
//     user_id: String,
//     answer_id: String,
// }

// #[rocket::async_trait]
// impl FromData for CommentAnswer {
//     type Error = String;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self>{
//         use Error::*;
//         use rocket::outcome::Outcome;


//         let comment_ct = ContentType::new("application", "x-C")
//     }

// }



#[post("/create_answer")]
fn create_comment_answer() -> String {
    let x : String = String::from("Criei Comentário para resposta");
    return x;
}

#[post("/create_question")]
fn create_comment_question() -> String {
    let x : String = String::from("Criei Comentário para pergunta");
    return x;
}

#[put("/edit")]
fn edit_comment() -> String {
    let x : String = String::from("Editei Comentário");
    return x;
}

#[delete("/delete")]
fn delete_comment() -> String {
    let x : String = String::from("Deletei Comentário");
    return x;
}

#[get("/all")]
fn get_all_comment() -> String {
    let x : String = String::from("Peguei todos os comentários de uma Q&A");
    return x;
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/comment", routes![create_comment_answer, create_comment_question, edit_comment, delete_comment, get_all_comment])
}
