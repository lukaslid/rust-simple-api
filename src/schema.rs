table! {
    user (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}
