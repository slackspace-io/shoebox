// @generated automatically by Diesel CLI.

diesel::table! {
    media_files (id) {
        id -> Nullable<Integer>,
        asset_type -> Text,
        path -> Nullable<Text>,
    }
}
