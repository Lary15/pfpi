-- Your SQL goes here
CREATE TABLE comments (
  id uuid NOT NULL,
  answer_id uuid,
  question_id uuid,
  user_id uuid NOT NULL,
  comment VARCHAR NOT NULL,
  PRIMARY KEY (id)
);