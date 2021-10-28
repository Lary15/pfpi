-- Your SQL goes here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  answer_id INTEGER,
  question_id INTEGER,
  user_id INTEGER NOT NULL,
  comment VARCHAR NOT NULL
);