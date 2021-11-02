-- Your SQL goes here

CREATE TABLE answers (
  id INTEGER PRIMARY KEY NOT NULL,
  body TEXT NOT NULL,
  user_id INTEGER NOT NULL,
  question_id INTEGER NOT NULL
)