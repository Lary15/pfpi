use super::oauth;

#[post("/login")]
pub fn login() -> &'static str {
  oauth::config_oauth_service();
}
