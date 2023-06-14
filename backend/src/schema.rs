// @generated automatically by Diesel CLI.

diesel::table! {
    chat (id) {
        id -> Uuid,
        #[max_length = 200]
        sender -> Varchar,
        #[max_length = 200]
        receiver -> Varchar,
        created_date -> Timestamp,
        #[max_length = 1000]
        content -> Varchar,
    }
}
