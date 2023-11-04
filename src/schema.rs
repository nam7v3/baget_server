// @generated automatically by Diesel CLI.

diesel::table! {
    expense_table (expense_id) {
        expense_id -> Int4,
        user_id -> Uuid,
        entry_date -> Text,
        amount -> Float4,
        expense -> Text,
    }
}

diesel::table! {
    transaction_table (transaction_id) {
        transaction_id -> Int4,
        user_id -> Uuid,
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

diesel::joinable!(expense_table -> users (user_id));
diesel::joinable!(transaction_table -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    expense_table,
    transaction_table,
    users,
);
