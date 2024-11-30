// @generated automatically by Diesel CLI.

diesel::table! {
    media_files (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}
