use super::cis2_db as db;
use crate::shared::{
    api::{ApiAddress, ApiContractAddress, ApiResult, PagedRequest, PagedResponse},
    db::DbConn,
};
use concordium_rust_sdk::{
    cis2,
    types::{Address, ContractAddress},
};
use itertools::Itertools;
use poem_openapi::{payload::Json, Object};

#[derive(Object)]
pub struct Token {
    pub token_id:          String,
    pub is_paused:         bool,
    pub metadata_url:      String,
    pub metadata_url_hash: String,
    pub supply:            String,
}

impl From<db::Token> for Token {
    fn from(value: db::Token) -> Self {
        Token {
            is_paused:         value.is_paused,
            metadata_url:      value.metadata_url,
            metadata_url_hash: value.metadata_hash.map(hex::encode).unwrap_or_default(),
            supply:            value.supply.to_string(),
            token_id:          value.token_id,
        }
    }
}

#[derive(Object)]
pub struct TokenHolder {
    pub token_id:       String,
    pub address:        ApiAddress,
    pub balance:        String,
    pub frozen_balance: String,
}

impl From<db::TokenHolder> for TokenHolder {
    fn from(token_holder: db::TokenHolder) -> Self {
        let address: ApiAddress = token_holder
            .holder_address
            .parse::<Address>()
            .expect("Error parsing holder address to address")
            .into();

        Self {
            token_id: token_holder.token_id,
            address,
            balance: token_holder.balance.to_string(),
            frozen_balance: token_holder.frozen_balance.to_string(),
        }
    }
}

#[derive(Object)]
pub struct Cis2Deposit {
    pub token_contract:   ApiContractAddress,
    pub token_id:         String,
    pub owner:            String,
    pub deposited_amount: String,
}

impl From<db::Cis2Deposit> for Cis2Deposit {
    fn from(value: db::Cis2Deposit) -> Self {
        let token_contract: ContractAddress =
            value.cis2_address.parse().expect("Error parsing contract address");

        Cis2Deposit {
            token_contract:   ApiContractAddress::from_contract_address(token_contract),
            token_id:         value.deposited_token_id,
            owner:            value.deposited_holder_address,
            deposited_amount: value.deposited_amount.to_string(),
        }
    }
}

pub fn tokens(
    conn: &mut DbConn,
    req: PagedRequest<ContractAddress>,
) -> ApiResult<PagedResponse<Token>> {
    let (tokens, page_count) =
        db::list_tokens_for_contract(conn, &req.data, req.page_size, req.page)?;
    let tokens: Vec<Token> = tokens.into_iter().map(|t| t.into()).collect_vec();
    let res = PagedResponse {
        data: tokens,
        page: req.page,
        page_count,
    };

    ApiResult::Ok(Json(res))
}

pub fn holders(
    conn: &mut DbConn,
    req: PagedRequest<(ContractAddress, Address)>,
) -> ApiResult<PagedResponse<TokenHolder>> {
    let (tokens, page_count) =
        db::list_tokens_by_holder(conn, &req.data.0, &req.data.1, req.page_size, req.page)?;
    let tokens: Vec<TokenHolder> = tokens.into_iter().map(|t| t.into()).collect_vec();
    let res = PagedResponse {
        data: tokens,
        page: req.page,
        page_count,
    };

    ApiResult::Ok(Json(res))
}

pub fn holders_of(
    conn: &mut DbConn,
    req: PagedRequest<(ContractAddress, cis2::TokenId)>,
) -> ApiResult<PagedResponse<TokenHolder>> {
    let (tokens, page_count) =
        db::list_holders_by_token(conn, &req.data.0, &req.data.1, req.page_size, req.page)?;

    let tokens: Vec<TokenHolder> = tokens.into_iter().map(|t| t.into()).collect_vec();
    let res = PagedResponse {
        data: tokens,
        page: req.page,
        page_count,
    };
    ApiResult::Ok(Json(res))
}

pub fn deposits_for_address(
    conn: &mut DbConn,
    req: PagedRequest<(ContractAddress, Address)>,
) -> ApiResult<PagedResponse<Cis2Deposit>> {
    let (tokens, page_count) =
        db::list_deposits_by_holder(conn, &req.data.0, &req.data.1, req.page_size, req.page)?;

    let tokens: Vec<Cis2Deposit> = tokens.into_iter().map(|t| t.into()).collect_vec();
    let res = PagedResponse {
        data: tokens,
        page: req.page,
        page_count,
    };
    ApiResult::Ok(Json(res))
}
