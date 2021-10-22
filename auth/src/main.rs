#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

mod api;
mod oauth;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/login", routes![api::login])
}