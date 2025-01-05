// @generated automatically by Diesel CLI.

diesel::table! {
    media (id) {
        id -> Int4,
        file_name -> Text,
        file_path -> Text,
        media_type -> Text,
        good_take -> Nullable<Bool>,
        highlight -> Nullable<Bool>,
        reviewed -> Nullable<Bool>,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        uploaded_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    media_people (media_id, person_id) {
        media_id -> Int4,
        person_id -> Int4,
    }
}

diesel::table! {
    media_tags (media_id, tag_id) {
        media_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(media_people -> media (media_id));
diesel::joinable!(media_people -> people (person_id));
diesel::joinable!(media_tags -> media (media_id));
diesel::joinable!(media_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    media,
    media_people,
    media_tags,
    people,
    tags,
);
