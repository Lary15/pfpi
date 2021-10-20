#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::Json};


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
