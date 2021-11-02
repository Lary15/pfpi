table! {
    answers (id) {
        id -> Integer,
        body -> Text,
        user_id -> Integer,
        question_id -> Integer,
    }
}

table! {
    questions (id) {
        id -> Integer,
        body -> Text,
        user_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
