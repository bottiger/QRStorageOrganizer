use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    groups (id) {
        id -> Integer,
        name -> Text,
        qr_salt -> Binary,
        qr_count -> Integer,
    }
}

table! {
    qrcode (id, group_id) {
        id -> Integer,
        group_id -> Integer,
        title -> Nullable<Text>,
        location -> Nullable<Text>,
        content -> Nullable<Text>,
        attachment -> Nullable<Binary>,
        version -> Integer,
    }
}

joinable!(qrcode -> groups (group_id));

allow_tables_to_appear_in_same_query!(
    groups,
    qrcode,
);