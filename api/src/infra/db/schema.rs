// @generated automatically by Diesel CLI.

diesel::table! {
    place (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        picture -> Varchar,
        #[max_length = 255]
        sub -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        whatsapp_number -> Nullable<Bigint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        picture -> Varchar,
        #[max_length = 255]
        sub -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    place,
    user,
);
