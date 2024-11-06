use events_listener::txn_processor::cis2_utils::ContractAddressToDecimal;
use events_listener::txn_processor::nft_multi_rewarded;
use events_listener::txn_processor::nft_multi_rewarded::db::NftMultiRewardedDetails;
use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use shared::db::DbPool;

use crate::api::*;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    /// Retrieves the nonce for the specified contract index and the authenticated account.
    ///
    /// # Arguments
    /// - `claims`: The authenticated account claims.
    /// - `contract_index`: The index of the contract to retrieve the nonce for.
    /// - `db_pool`: The database connection pool.
    ///
    /// # Returns
    /// The nonce for the specified contract index and authenticated account.
    #[oai(
        path = "/tree_nft/contract/self_nonce",
        method = "get",
        tag = "ApiTags::TreeNft"
    )]
    pub async fn nonce(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contract): Data<&TreeNftContractAddress>,
    ) -> JsonResult<u64> {
        let mut conn = db_pool.get()?;
        let account = ensure_account_registered(&claims)?;
        let account_nonce = nft_multi_rewarded::db::AddressNonce::find(
            &mut conn,
            contract.0.to_decimal(),
            &account.into(),
        )?
        .map(|a| a.nonce)
        .unwrap_or(0);
        Ok(Json(account_nonce as u64))
    }
}

#[derive(Clone, Copy)]
pub struct AdminApi;

#[OpenApi]
impl AdminApi {
    #[oai(
        path = "/admin/tree_nft/contract",
        method = "get",
        tag = "ApiTags::TreeNft"
    )]
    pub async fn details(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contract): Data<&TreeNftContractAddress>,
    ) -> JsonResult<NftMultiRewardedDetails> {
        ensure_is_admin(&claims)?;
        let contract = NftMultiRewardedDetails::find(&mut db_pool.get()?, contract.0.to_decimal())?
            .ok_or(Error::NotFound(PlainText("Contract not found".to_string())))?;
        Ok(Json(contract))
    }
}
