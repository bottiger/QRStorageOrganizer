table! {
    qrcodes (id) {
        id -> Integer,
        qr -> BigInt,
        title -> Nullable<Text>,
        body -> Nullable<Text>,
        images -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
