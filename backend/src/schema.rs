// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        github_data -> Nullable<Json>,
        extra_data -> Nullable<Json>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
