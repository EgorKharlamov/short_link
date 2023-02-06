// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        shared_link -> Text,
        original_link -> Text,
        created_at -> Timestamp,
    }
}
