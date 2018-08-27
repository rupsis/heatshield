table! {
    use diesel::sql_types::*;

    accounts (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        email -> Nullable<Text>,
        verification_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;

    clients (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    client_tokens (id) {
        id -> Int4,
        client_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    confirmations (id) {
        id -> Int4,
        code -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    salts (id) {
        id -> Int4,
        salt -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    user_tokens (id) {
        id -> Int4,
        client_id -> Uuid,
        account_id -> Uuid,
        refresh_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    verifications (id) {
        id -> Int4,
        verified_at -> Nullable<Timestamp>,
        ip_address -> Text,
        confirmation_id -> Int4,
    }
}

joinable!(accounts -> verifications (verification_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    clients,
    client_tokens,
    confirmations,
    salts,
    user_tokens,
    verifications,
);
