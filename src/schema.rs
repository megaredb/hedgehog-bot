// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        boosty_id -> Int8,
        expires_at -> Timestamp,
    }
}
