use rocket::serde::Serialize;
use crate::models::LoadComment;

#[derive(Serialize)]
pub struct ResponseComment {
  pub user_id: i32,
  pub qa_id: i32,
  pub id: i32,
  pub comment: String,
}

#[derive(Serialize)]
pub struct EditResponse<'r> {
  pub message: &'r str,
}

#[derive(Serialize)]
pub struct DeleteResponse {
  pub message: &'static str,
}

#[derive(Serialize)]
pub struct AllComment {
  pub id: i32,
  pub message: &'static str,
}

#[derive(Serialize)]
pub struct GetAllResponse {
  pub comments: Vec<LoadComment>,
}
