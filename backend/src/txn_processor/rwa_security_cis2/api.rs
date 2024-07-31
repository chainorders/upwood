use super::db;
use crate::shared::{
    api::{ApiAddress, ApiContractAddress, ApiResult, PagedResponse, PAGE_SIZE},
    db::DbPool,
};
use concordium_rust_sdk::{
    cis2,
    types::{Address, ContractAddress},
};
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

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

#[derive(Clone, Copy)]
pub struct Cis2Api;

#[OpenApi]
impl Cis2Api {
    #[oai(path = "/rwa-security-cis2/:index/:subindex/tokens/:page", method = "get")]
    pub async fn tokens(
        &self,
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<i64>,
    ) -> ApiResult<PagedResponse<Token>> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_tokens_for_contract(&mut conn, &cis2_address, PAGE_SIZE, page)?;
        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };

        ApiResult::Ok(Json(res))
    }

    #[oai(path = "/rwa-security-cis2/:index/:subindex/holders/:address/:page", method = "get")]
    pub async fn holders(
        self,
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<i64>,
    ) -> ApiResult<PagedResponse<TokenHolder>> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let holder_address: Address = address.parse()?;
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_tokens_by_holder(&mut conn, &cis2_address, &holder_address, PAGE_SIZE, page)?;
        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };

        ApiResult::Ok(Json(res))
    }

    #[oai(path = "/rwa-security-cis2/:index/:subindex/holdersOf/:token_id/:page", method = "get")]
    pub async fn holders_of(
        &self,
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<i64>,
    ) -> ApiResult<PagedResponse<TokenHolder>> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let token_id: cis2::TokenId = token_id.parse()?;
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_holders_by_token(&mut conn, &cis2_address, &token_id, PAGE_SIZE, page)?;

        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };
        ApiResult::Ok(Json(res))
    }

    #[oai(path = "/rwa-security-cis2/:index/:subindex/deposited/:owner/:page", method = "get")]
    pub async fn deposited(
        &self,
        Data(pool): Data<&DbPool>,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<i64>,
    ) -> ApiResult<PagedResponse<Cis2Deposit>> {
        let cis2_address = ContractAddress {
            index,
            subindex,
        };
        let owner: Address = owner.parse()?;
        let mut conn = pool.get()?;
        let (tokens, page_count) =
            db::list_deposits_by_holder(&mut conn, &cis2_address, &owner, PAGE_SIZE, page)?;

        let tokens = tokens.into_iter().map(|t| t.into()).collect();
        let res = PagedResponse {
            data: tokens,
            page,
            page_count,
        };
        ApiResult::Ok(Json(res))
    }
}
