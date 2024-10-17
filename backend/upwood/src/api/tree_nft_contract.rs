use std::sync::Arc;

use concordium::account::Signer;
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use concordium_rust_sdk::types::{ContractAddress, WalletAccount};
use events_listener::txn_processor::{cis2_security, nft_multi_rewarded};
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use shared::api::PagedResponse;
use shared::db::DbPool;
use tree_nft_metadata::MetadataUrl;

use super::PAGE_SIZE;
use crate::api::*;
use crate::db;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/tree_nft/contract/:contract_index/nonce", method = "get")]
    pub async fn nonce(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Path(contract_index): Path<u64>,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<u64> {
        let mut conn = db_pool.get()?;
        let account = db::users::find_user_by_cognito_user_id(&mut conn, &claims.sub)?
            .ok_or(Error::NotFound(PlainText("User not found".to_string())))?
            .account_address()
            .ok_or(Error::NotFound(PlainText(
                "User not registered".to_string(),
            )))?;
        let account_nonce = nft_multi_rewarded::db::find_address_nonce(
            &mut conn,
            &ContractAddress::new(contract_index, 0),
            &account.into(),
        )?
        .unwrap_or(0);
        Ok(Json(account_nonce as u64))
    }

    #[oai(
        path = "/tree_nft/contract/:contract_index/metadata/random",
        method = "get"
    )]
    pub async fn metadata_get_random(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&TreeNftConfig>,
        Path(contract_index): Path<u64>,
    ) -> JsonResult<MintData> {
        let mut conn = db_pool.get()?;
        let account = db::users::find_user_by_cognito_user_id(&mut conn, &claims.sub)?
            .ok_or(Error::NotFound(PlainText("User not found".to_string())))?
            .account_address()
            .ok_or(Error::NotFound(PlainText(
                "User not registered".to_string(),
            )))?;
        let account_nonce = nft_multi_rewarded::db::find_address_nonce(
            &mut conn,
            &ContractAddress::new(contract_index, 0),
            &account.into(),
        )?
        .unwrap_or(0);

        let metadata = db::tree_nft_metadata::find_random(&mut conn)?
            .ok_or(Error::NotFound(PlainText("No metadata found".to_string())))?;
        let metadata = SignedMetadata {
            contract_address: ::nft_multi_rewarded::types::ContractAddress::new(contract_index, 0)
                .to_string(),
            metadata_url:     MetadataUrl {
                url:  metadata.metadata_url,
                hash: metadata.metadata_hash,
            },
            account:          account.to_string(),
            account_nonce:    account_nonce as u64,
        };
        let signature = hash_and_sign(&metadata, &config.agent)?;
        let signature = serde_json::to_value(signature).map_err(|_| {
            Error::InternalServer(PlainText("Failed to serialize signature".to_string()))
        })?;
        Ok(Json(MintData {
            signed_metadata: metadata,
            signer: config.agent.address().to_string(),
            signature,
        }))
    }

    #[oai(
        path = "/tree_nft/contract/:contract_index/list_owners/:metadata_id",
        method = "get"
    )]
    pub async fn metadata_list_owners(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(contract_index): Path<u64>,
        Path(metadata_id): Path<String>,
        Query(page): Query<i64>,
    ) -> JsonResult<PagedResponse<cis2_security::api::TokenHolder>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::find(&mut conn, &metadata_id)?
            .ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;
        let (holders, page_count) = cis2_security::db::list_holders_by_token_metadata_url(
            &mut conn,
            &ContractAddress::new(contract_index, 0),
            &metadata.metadata_url,
            PAGE_SIZE,
            page,
        )?;
        Ok(Json(PagedResponse::into_new(holders, page, page_count)))
    }
}

fn hash_and_sign(metadata: &SignedMetadata, agent: &TreeNftAgent) -> Result<AccountSignatures> {
    let hash = metadata.hash(hasher)?;
    let signature = agent.sign(&hash);
    Ok(signature)
}

#[derive(Object, Debug)]
pub struct MintData {
    pub signed_metadata: SignedMetadata,
    pub signer:          String,
    /// Json serialized `AccountSignatures`
    pub signature:       serde_json::Value,
}

#[derive(Object, Debug)]
pub struct SignedMetadata {
    pub contract_address: String,
    pub metadata_url:     MetadataUrl,
    pub account:          String,
    pub account_nonce:    u64,
}

impl SignedMetadata {
    pub fn hash<T>(&self, hasher: T) -> std::result::Result<[u8; 32], HashError>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let internal = ::nft_multi_rewarded::SignedMetadata {
            contract_address: self
                .contract_address
                .parse()
                .map_err(|_| HashError::ContractParse)?,
            account:          self.account.parse().map_err(|_| HashError::AccountParse)?,
            metadata_url:     ::nft_multi_rewarded::MetadataUrl {
                url:  self.metadata_url.url.clone(),
                hash: self
                    .metadata_url
                    .hash
                    .as_ref()
                    .map(hex::decode)
                    .transpose()
                    .map_err(|_| HashError::MetadataUrlHexDecode)?
                    .map(|hash| hash.try_into())
                    .transpose()
                    .map_err(|_| HashError::MetadataUrlHexDecode)?,
            },
            account_nonce:    self.account_nonce,
        };
        let hash = internal.hash(hasher).map_err(|_| HashError::Hash)?;
        Ok(hash)
    }
}

pub struct TreeNftAgent(pub WalletAccount);
impl Signer for TreeNftAgent {
    fn wallet(&self) -> &WalletAccount { &self.0 }
}

#[derive(Debug)]
pub enum HashError {
    ContractParse,
    AccountParse,
    MetadataUrlHexDecode,
    Hash,
}

impl From<HashError> for Error {
    fn from(val: HashError) -> Self {
        Error::InternalServer(PlainText(format!("Hash Error: {val:?}")))
    }
}

#[derive(Clone)]
pub struct TreeNftConfig {
    pub agent: Arc<TreeNftAgent>,
}
