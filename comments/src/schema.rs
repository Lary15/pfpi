table! {
    comments (id) {
        id -> Int4,
        answer_id -> Int4,
        question_id -> Int4,
        user_id -> Int4,
        comment -> Varchar,
    }
}
