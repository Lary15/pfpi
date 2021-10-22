use oauth2::{
  AuthorizationCode,
  AuthUrl,
  ClientId,
  ClientSecret,
  CsrfToken,
  PkceCodeChallenge,
  RedirectUrl,
  Scope,
  TokenResponse,
  TokenUrl
};
use oauth2::basic::BasicClient;

pub fn config_oauth_service() {
  BasicClient::new(
    ClientId::new(dotenv!("OAUTH_CLIENT_ID").to_string()),
    Some(ClientSecret::new(dotenv!("OAUTH_CLIENT_SECRET").to_string())),
    AuthUrl::new(dotenv!("OAUTH_AUTHORIZE_REQUEST_URL").to_string())?,
    Some(TokenUrl::new(dotenv!("OAUTH_TOKEN_URL").to_string())?)
  )
}

pub fn generate_complete_auth_url(BasicClient client, PkceCodeChallenge pkce_challenge) {
  client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new("read".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();
}