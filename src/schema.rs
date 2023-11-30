// @generated automatically by Diesel CLI.

diesel::table! {
    account_table (account_id, user_id) {
        account_id -> Int4,
        user_id -> Uuid,
        account -> Text,
        balance -> Float4,
        income -> Float4,
        expense -> Float4,
    }
}

diesel::table! {
    schedule_table (transaction_id, user_id) {
        transaction_id -> Int4,
        user_id -> Uuid,
        time_unit -> Text,
        time_schedule -> Int4,
        last_time_added -> Int4,
    }
}

diesel::table! {
    transaction_table (_timestamp, user_id) {
        user_id -> Uuid,
        _timestamp -> Int4,
        entry_date -> Text,
        amount -> Float4,
        account -> Text,
        category -> Text,
        transaction_type -> Text,
        transaction_title -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        password -> Varchar,
        verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(account_table -> users (user_id));
diesel::joinable!(schedule_table -> users (user_id));
diesel::joinable!(transaction_table -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_table,
    schedule_table,
    transaction_table,
    users,
);
