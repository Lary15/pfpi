use crate::schema::*;
use rocket::serde::Serialize;


#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment <'a> {
  pub answer_id: i32 ,
  pub question_id: i32,
  pub user_id: i32,
  pub comment: &'a str,
}


#[derive(Queryable, Serialize)]
pub struct LoadComment {
  pub id: i32,
  pub answer_id: i32 ,
  pub question_id: i32,
  pub user_id: i32,
  pub comment:  String,
}
