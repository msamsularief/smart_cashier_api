table! {
    access_tokens (id) {
        id -> Int8,
        account_id -> Nullable<Int8>,
        token -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    account_passhash (id) {
        id -> Int8,
        account_id -> Int8,
        passhash -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    accounts (id) {
        id -> Int8,
        full_name -> Varchar,
        phone_num -> Varchar,
        email -> Varchar,
        role -> Text,
        active -> Bool,
        registration_time -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int4,
        role_level -> Int2,
        role_name -> Varchar,
    }
}

joinable!(access_tokens -> accounts (account_id));
joinable!(account_passhash -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    account_passhash,
    accounts,
    roles,
);
