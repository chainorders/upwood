// @generated automatically by Diesel CLI.

diesel::table! {
    tree_nft_metadatas (id) {
        #[max_length = 16]
        id -> Bpchar,
        metadata_url -> Text,
        #[max_length = 64]
        metadata_hash -> Nullable<Bpchar>,
        probablity_percentage -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_affiliate_accounts (account_address) {
        #[max_length = 255]
        account_address -> Varchar,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_affiliates (id) {
        id -> Int4,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        #[max_length = 255]
        affiliate_account_address -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_challenges (id) {
        id -> Int4,
        #[max_length = 255]
        cognito_user_id -> Varchar,
        challenge -> Bytea,
        #[max_length = 255]
        account_address -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (cognito_user_id) {
        #[max_length = 255]
        cognito_user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        account_address -> Nullable<Varchar>,
        desired_investment_amount -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(user_affiliate_accounts -> users (cognito_user_id));
diesel::joinable!(user_affiliates -> users (cognito_user_id));
diesel::joinable!(user_challenges -> users (cognito_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tree_nft_metadatas,
    user_affiliate_accounts,
    user_affiliates,
    user_challenges,
    users,
);
