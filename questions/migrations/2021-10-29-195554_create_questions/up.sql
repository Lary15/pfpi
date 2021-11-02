-- Your SQL goes here

CREATE TABLE questions (
  id INTEGER  PRIMARY KEY,
  body TEXT,
  user_id INTEGER NOT NULL
)