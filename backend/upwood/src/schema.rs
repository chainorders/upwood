// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "forest_project_state"))]
    pub struct ForestProjectState;
}

diesel::table! {
    forest_project_legal_contract_user_signatures (project_id, cognito_user_id) {
        project_id -> Uuid,
        cognito_user_id -> Varchar,
        user_account -> Varchar,
        user_signature -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_legal_contracts (project_id) {
        project_id -> Uuid,
        text_url -> Varchar,
        edoc_url -> Varchar,
        pdf_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_notifications (id) {
        id -> Uuid,
        project_id -> Uuid,
        cognito_user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_prices (project_id, price_at) {
        project_id -> Uuid,
        price -> Numeric,
        price_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    forest_project_property_media (id) {
        id -> Uuid,
        project_id -> Uuid,
        image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ForestProjectState;

    forest_projects (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> ForestProjectState,
        image_small_url -> Varchar,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        contract_address -> Numeric,
        p2p_trade_contract_address -> Nullable<Numeric>,
        mint_fund_contract_address -> Nullable<Numeric>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Text,
        property_media_footer -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        id -> Uuid,
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

diesel::joinable!(forest_project_legal_contract_user_signatures -> forest_projects (project_id));
diesel::joinable!(forest_project_legal_contract_user_signatures -> users (cognito_user_id));
diesel::joinable!(forest_project_legal_contracts -> forest_projects (project_id));
diesel::joinable!(forest_project_notifications -> forest_projects (project_id));
diesel::joinable!(forest_project_notifications -> users (cognito_user_id));
diesel::joinable!(forest_project_prices -> forest_projects (project_id));
diesel::joinable!(forest_project_property_media -> forest_projects (project_id));
diesel::joinable!(user_affiliate_accounts -> users (cognito_user_id));
diesel::joinable!(user_affiliates -> users (cognito_user_id));
diesel::joinable!(user_challenges -> users (cognito_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    forest_project_legal_contract_user_signatures,
    forest_project_legal_contracts,
    forest_project_notifications,
    forest_project_prices,
    forest_project_property_media,
    forest_projects,
    tree_nft_metadatas,
    user_affiliate_accounts,
    user_affiliates,
    user_challenges,
    users,
);
