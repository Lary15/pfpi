use anyhow::Result;

use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};

pub fn config_oauth_service_client() -> Result<BasicClient> {
  let client = BasicClient::new(
    ClientId::new(dotenv!("OAUTH_CLIENT_ID").to_string()),
    Some(ClientSecret::new(
      dotenv!("OAUTH_CLIENT_SECRET").to_string(),
    )),
    AuthUrl::new(dotenv!("OAUTH_AUTHORIZE_REQUEST_URL").to_string())?,
    Some(TokenUrl::new(dotenv!("OAUTH_TOKEN_URL").to_string())?),
  )
  .set_redirect_uri(
    RedirectUrl::new("http://localhost:8000/auth/login/redirect".to_string())?
  );

  Ok(client)
}

pub fn generate_auth_url(client: BasicClient) -> (Url, CsrfToken) {
  client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new("user:email".to_string()))
    .url()
}
