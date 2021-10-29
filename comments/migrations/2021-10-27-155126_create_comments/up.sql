-- Your SQL goes here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  answer_id INTEGER NOT NULL,
  question_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  comment VARCHAR NOT NULL
);