use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn connection_db() -> PgConnection {

  dotenv().ok();
  
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

  return PgConnection::establish(&database_url)
    .unwrap_or_else( |_| panic!("Couldn't establish connection to the database {}", database_url));

}