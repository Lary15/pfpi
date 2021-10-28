use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct Comment<'r> {
  pub message: &'r str,
}

#[derive(Deserialize)]
pub struct EditComment<'r> {
  pub id: i32,
  pub message: &'r str,
}

#[derive(Deserialize)]
pub struct GetAllComments {
  pub id: i32,
}
