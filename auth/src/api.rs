use super::oauth;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use oauth2::reqwest::async_http_client;
use oauth2::AuthorizationCode;

#[derive(Serialize, Deserialize)]
pub struct AuthRes {
  pub user_redirect_url: String,
}

#[post("/login")]
pub fn login() -> Json<AuthRes> {
  let client = oauth::config_oauth_service_client().expect("Error while building oauth client!");

  let (authorize_url, _) = oauth::generate_auth_url(client);

  Json(AuthRes {
    user_redirect_url: authorize_url.to_string(),
  })
}

#[get("/login/redirect?<code>")]
pub async fn login_redirect(code: String) -> Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
  let client = oauth::config_oauth_service_client().expect("Error while building oauth client!");

  let token_res = client
    .exchange_code(AuthorizationCode::new(code))
    .request_async(async_http_client)
    .await
    .unwrap();

  Json(token_res)
}
