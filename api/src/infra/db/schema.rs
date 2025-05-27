// @generated automatically by Diesel CLI.

diesel::table! {
    appointment (user_id, place_id) {
        date -> Date,
        user_id -> Bigint,
        place_id -> Bigint,
    }
}

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

diesel::joinable!(appointment -> place (place_id));
diesel::joinable!(appointment -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(appointment, place, user,);
