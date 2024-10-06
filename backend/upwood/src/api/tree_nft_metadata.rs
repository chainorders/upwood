use std::sync::Arc;

use chrono::{DateTime, Utc};
use concordium::account::Signer;
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use concordium_rust_sdk::types::WalletAccount;
use events_listener::txn_processor::cis2_security;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{Object, OpenApi};
use shared::api::PagedResponse;
use shared::db::DbPool;

use crate::api::*;
use crate::db;
use crate::utils::*;

pub const PAGE_SIZE: i64 = 20;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/tree_nft/metadata", method = "post")]
    pub async fn metadata_insert(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<AddMetadataRequest>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let metadata = db::tree_nft_metadata::TreeNftMetadataInsert::new(
            req.metadata_url()?,
            req.probablity_percentage()?,
        );
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::insert(&mut conn, &metadata)?.ok_or(
            Error::BadRequest(PlainText("Failed to insert metadata".to_string())),
        )?;
        Ok(Json(metadata.into()))
    }

    #[oai(path = "/tree_nft/nonce", method = "get")]
    pub async fn nonce(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&TreeNftConfig>,
        Data(nft_multi_rewarded_api): Data<
            &events_listener::txn_processor::nft_multi_rewarded::api::Api,
        >,
    ) -> JsonResult<u64> {
        let account = ensure_account_registered(&claims)?;
        let Json(account_nonce) = nft_multi_rewarded_api
            .nonce(
                Path(config.contract.clone()),
                Path(account.to_string()),
                Data(db_pool),
            )
            .await
            .map_err(|_| Error::InternalServer(PlainText("Failed to get nonce".to_string())))?;
        Ok(Json(account_nonce as u64))
    }

    #[oai(path = "/tree_nft/metadata/random", method = "get")]
    pub async fn metadata_get_random(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&TreeNftConfig>,
        Data(nft_multi_rewarded_api): Data<
            &events_listener::txn_processor::nft_multi_rewarded::api::Api,
        >,
    ) -> JsonResult<MintData> {
        let account = ensure_account_registered(&claims)?;
        let Json(account_nonce) = nft_multi_rewarded_api
            .nonce(
                Path(config.contract.clone()),
                Path(account.to_string()),
                Data(db_pool),
            )
            .await
            .map_err(|_| Error::InternalServer(PlainText("Failed to get nonce".to_string())))?;

        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::find_random(&mut conn)?
            .ok_or(Error::NotFound(PlainText("No metadata found".to_string())))?;
        let metadata = SignedMetadata {
            contract_address: config.contract.clone(),
            metadata_url:     metadata.into(),
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

    #[oai(path = "/tree_nft/metadata/list", method = "get")]
    pub async fn metadata_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
    ) -> JsonResult<Vec<TreeNftMetadata>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::list(&mut conn, PAGE_SIZE, page)?;
        Ok(Json(metadata.into_iter().map(|m| m.into()).collect()))
    }

    #[oai(path = "/tree_nft/metadata/:id/list_owners", method = "get")]
    pub async fn metadata_list_owners(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&TreeNftConfig>,
        Data(nft_multi_rewarded_api): Data<
            &events_listener::txn_processor::nft_multi_rewarded::api::Api,
        >,
        Path(id): Path<String>,
        Query(page): Query<i64>,
    ) -> JsonResult<PagedResponse<cis2_security::api::TokenHolder>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::find(&mut conn, &id)?
            .ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;
        let Json(holders) = nft_multi_rewarded_api
            .holders_by_token_metadata_url(
                Path(config.contract.clone()),
                Path(metadata.metadata_url),
                Query(page),
                Data(db_pool),
            )
            .await
            .map_err(|_| Error::InternalServer(PlainText("Failed to get holders".to_string())))?;
        Ok(Json(holders))
    }

    #[oai(path = "/tree_nft/metadata/:id", method = "get")]
    pub async fn metadata_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = db::tree_nft_metadata::find(&mut conn, &id)?;
        let metadata =
            metadata.ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;

        Ok(Json(metadata.into()))
    }

    #[oai(path = "/tree_nft/metadata/:id", method = "delete")]
    pub async fn metadata_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let row_count = db::tree_nft_metadata::delete(&mut conn, &id)?;
        if row_count != 1 {
            return Err(Error::NotFound(PlainText("Metadata not found".to_string())));
        }
        Ok(())
    }
}

fn hash_and_sign(metadata: &SignedMetadata, agent: &TreeNftAgent) -> Result<AccountSignatures> {
    let hash = metadata.hash(hasher)?;
    let signature = agent.sign(&hash);
    Ok(signature)
}

pub struct TreeNftAgent(pub WalletAccount);
impl Signer for TreeNftAgent {
    fn wallet(&self) -> &WalletAccount { &self.0 }
}

#[derive(Clone)]
pub struct TreeNftConfig {
    pub contract: String,
    pub agent:    Arc<TreeNftAgent>,
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

#[derive(Object, Debug)]
pub struct MetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

impl From<db::tree_nft_metadata::TreeNftMetadata> for MetadataUrl {
    fn from(value: db::tree_nft_metadata::TreeNftMetadata) -> Self {
        Self {
            url:  value.metadata_url,
            hash: value.metadata_hash,
        }
    }
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

impl SignedMetadata {
    pub fn hash<T>(&self, hasher: T) -> std::result::Result<[u8; 32], HashError>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let internal = nft_multi_rewarded::SignedMetadata {
            contract_address: self
                .contract_address
                .parse()
                .map_err(|_| HashError::ContractParse)?,
            account:          self.account.parse().map_err(|_| HashError::AccountParse)?,
            metadata_url:     nft_multi_rewarded::MetadataUrl {
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

#[derive(Object, Debug)]
pub struct AddMetadataRequest {
    pub metadata_url:          MetadataUrl,
    /// The probability of the metadata to be chosen for minting
    /// between 1 and 100
    pub probablity_percentage: i16,
}

impl AddMetadataRequest {
    pub fn probablity_percentage(&self) -> Result<i16> {
        if self.probablity_percentage < 1 || self.probablity_percentage > 100 {
            return Err(Error::BadRequest(PlainText(
                "Probablity percentage must be between 1 and 100".to_string(),
            )));
        }

        Ok(self.probablity_percentage)
    }

    pub fn metadata_url(&self) -> Result<nft_multi_rewarded::MetadataUrl> {
        Ok(nft_multi_rewarded::MetadataUrl {
            url:  self.metadata_url.url.clone(),
            hash: self
                .metadata_url
                .hash
                .as_ref()
                .map(hex::decode)
                .transpose()
                .map_err(|_| {
                    Error::BadRequest(PlainText(
                        "Metadata hash must be a valid hex string".to_string(),
                    ))
                })?
                .map(|hash| hash.try_into())
                .transpose()
                .map_err(|_| {
                    Error::BadRequest(PlainText(
                        "Metadata hash must be a valid hex string".to_string(),
                    ))
                })?,
        })
    }
}

#[derive(Object, Debug)]
pub struct TreeNftMetadata {
    pub id:                    String,
    pub metadata_url:          String,
    pub metadata_hash:         Option<String>,
    pub probablity_percentage: i16,
    pub created_at:            DateTime<Utc>,
}

impl From<db::tree_nft_metadata::TreeNftMetadata> for TreeNftMetadata {
    fn from(value: db::tree_nft_metadata::TreeNftMetadata) -> Self {
        Self {
            id:                    value.id,
            metadata_url:          value.metadata_url,
            metadata_hash:         value.metadata_hash,
            probablity_percentage: value.probablity_percentage,
            created_at:            value.created_at.and_utc(),
        }
    }
}
