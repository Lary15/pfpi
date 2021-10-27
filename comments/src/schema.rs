table! {
    comments (id) {
        id -> Uuid,
        answer_id -> Nullable<Uuid>,
        question_id -> Nullable<Uuid>,
        user_id -> Uuid,
        comment -> Varchar,
    }
}
