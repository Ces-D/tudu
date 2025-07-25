// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        color -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        project_id -> Integer,
        parent_id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        status -> Integer,
        priority -> Integer,
        due_date -> Nullable<Timestamp>,
        estimated_minutes -> Nullable<Integer>,
        location -> Nullable<Text>,
        url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(todos -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, todos,);
