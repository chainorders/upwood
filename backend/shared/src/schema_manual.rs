use crate::schema::{
    forest_project_legal_contract_user_signatures, forest_project_legal_contracts,
    forest_project_token_contracts,
};

diesel::table! {
    user_transactions (transaction_hash) {
        transaction_hash -> Varchar,
        block_height -> Numeric,
        forest_project_id -> Uuid,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        currency_amount -> Numeric,
        cognito_user_id -> Varchar,
        transaction_type -> Varchar,
        account_address -> Varchar,
    }
}

diesel::table! {
    forest_project_supply (forest_project_id) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        supply -> Nullable<Numeric>,
        symbol -> Varchar,
        decimals -> Integer,
    }
}

diesel::table! {
    forest_project_user_balance_agg (cognito_user_id, forest_project_id) {
        cognito_user_id -> Varchar,
        forest_project_id -> Uuid,
        total_balance -> Numeric,
    }
}

diesel::table! {
    forest_project_token_contract_user_balance_agg (forest_project_id, cognito_user_id, contract_address) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        forest_project_name -> Varchar,
        cognito_user_id -> Varchar,
        contract_address -> Numeric,
        contract_type -> crate::schema::sql_types::ForestProjectSecurityTokenContractType,
        token_symbol -> Varchar,
        token_decimals -> Integer,
        total_balance -> Numeric,
        un_frozen_balance -> Numeric,
    }
}

diesel::table! {
    holder_yields (yielder_contract_address, holder_address, token_id, yield_contract_address, yield_token_id) {
        yielder_contract_address -> Numeric,
        holder_address -> Varchar,
        un_frozen_balance -> Numeric,
        forest_project_id -> Uuid,
        cis2_address -> Numeric,
        token_id -> Numeric,
        token_ver_to -> Numeric,
        previous_yield_token_id -> Nullable<Numeric>,
        yield_contract_address -> Numeric,
        yield_token_id -> Numeric,
        yield_rate_numerator -> Numeric,
        yield_rate_denominator -> Numeric,
        yield_type -> Varchar,
        yield_period -> Numeric,
        yield_value -> Numeric,
    }
}

diesel::table! {
    affiliate_claims (id) {
        forest_project_id -> Uuid,
        contract_address -> Numeric,
        id -> Uuid,
        block_height -> Numeric,
        txn_index -> Numeric,
        token_contract_address -> Numeric,
        token_id -> Numeric,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        account_address -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric,
        create_time -> Timestamp,
        user_cognito_user_id -> Varchar,
        user_email -> Varchar,
        affiliate_account_address -> Varchar,
        affiliate_cognito_user_id -> Varchar,
        affiliate_commission -> Numeric,
        affiliate_reward -> Numeric,
        claim_nonce -> Nullable<Numeric>,
        claim_amount -> Nullable<Numeric>,
        affiliate_remaining_reward -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    forest_project_legal_contracts,
    forest_project_user_balance_agg
);

diesel::allow_tables_to_appear_in_same_query!(
    forest_project_legal_contract_user_signatures,
    forest_project_user_balance_agg,
);

diesel::allow_tables_to_appear_in_same_query!(holder_yields, forest_project_token_contracts,);
