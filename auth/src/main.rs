#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

mod api;
mod oauth;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/auth", routes![api::login, api::login_redirect])
}