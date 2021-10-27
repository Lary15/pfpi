use uuid::Uuid;
use diesel::{Queryable, Insertable};
use crate::schema::comments;

#[derive(Queryable)]
pub struct Comment {
  pub id: Uuid,
  pub answer_id: Uuid,
  pub question_id: Uuid,
  pub user_id: Uuid,
  pub comment: String,
}


#[derive(Insertable)]
#[table_name = "comments"]
pub struct Comment_Answer <'a> {
  pub id: Uuid,
  pub answer_id: Uuid,
  pub user_id: Uuid,
  pub comment: & 'a str,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct Comment_Question <'a> {
  pub id: Uuid,
  pub question_id: Uuid,
  pub user_id: Uuid,
  pub comment: & 'a str,
}