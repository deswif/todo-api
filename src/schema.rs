// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        is_complete -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(todo -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo,
    users,
);
