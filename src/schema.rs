table! {
    user (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
