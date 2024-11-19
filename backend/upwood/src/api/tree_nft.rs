use concordium::account::Signer;
use concordium_rust_sdk::base::contracts_common::AccountSignatures;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db::cis2_security::{list_holders_by_token_metadata_url, TokenHolder};
use shared::db::nft_multi_rewarded::{AddressNonce, NftMultiRewardedDetails};
use shared::db::security_sft_single::TokenDetails;
use shared::db_app::tree_nft_metadata::TreeNftMetadata;

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
    pub async fn tree_nft_contract_self_nonce(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<u64> {
        let mut conn = db_pool.get()?;
        let account = ensure_account_registered(&claims)?;
        let account_nonce = AddressNonce::find(
            &mut conn,
            contracts.tree_nft_contract_index,
            &account.into(),
        )?
        .map(|a| a.nonce)
        .unwrap_or(0);
        Ok(Json(account_nonce as u64))
    }

    #[oai(
        path = "/admin/tree_nft/contract",
        method = "get",
        tag = "ApiTags::TreeNft"
    )]
    pub async fn admin_tree_nft_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<NftMultiRewardedDetails> {
        ensure_is_admin(&claims)?;
        let contract =
            NftMultiRewardedDetails::find(&mut db_pool.get()?, contracts.tree_nft_contract_index)?
                .ok_or(Error::NotFound(PlainText("Contract not found".to_string())))?;
        Ok(Json(contract))
    }

    /// Retrieves the details of the token associated with the TreeFT contract.
    ///
    /// This function is an administrative endpoint that requires the caller to be an admin.
    /// It retrieves the token details from the database using the provided `DbPool` and `TreeFTContractAddress`.
    ///
    /// # Arguments
    /// - `db_pool`: A reference to the database connection pool.
    /// - `claims`: The bearer authorization claims of the caller.
    /// - `carbon_credit_contract`: A reference to the TreeFT contract address.
    ///
    /// # Returns
    /// A `JsonResult` containing the `TokenDetails` of the token associated with the TreeFT contract.
    #[oai(
        path = "/admin/tree_fts/contract",
        method = "get",
        tag = "ApiTags::TreeFT"
    )]
    pub async fn admin_tree_ft_contract(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<TokenDetails> {
        ensure_is_admin(&claims)?;
        let token = TokenDetails::find(contracts.tree_ft_contract_index, &mut db_pool.get()?)?
            .ok_or(Error::NotFound(PlainText("Token not found".to_string())))?;
        Ok(Json(token))
    }

    /// Retrieves a random metadata entry and generates a signed metadata object for minting a new NFT.
    ///
    /// # Arguments
    /// - `claims`: The authenticated account claims.
    /// - `db_pool`: The database connection pool.
    /// - `config`: The TreeNftConfig instance.
    /// - `contract_index`: The index of the contract to retrieve the metadata for.
    ///
    /// # Returns
    /// A `MintData` object containing the signed metadata and the signer's address and signature.
    #[oai(
        path = "/tree_nft/metadata/random",
        method = "get",
        tag = "ApiTags::TreeNftMetadata"
    )]
    pub async fn tree_nft_metadata_get_random(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(config): Data<&TreeNftConfig>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<MintData> {
        let mut conn = db_pool.get()?;
        let account = ensure_account_registered(&claims)?;
        let account_nonce = AddressNonce::find(
            &mut conn,
            contracts.tree_nft_contract_index,
            &account.into(),
        )?
        .map(|a| a.nonce)
        .unwrap_or(0);

        let metadata = TreeNftMetadata::find_random(&mut conn)?
            .ok_or(Error::NotFound(PlainText("No metadata found".to_string())))?;
        let metadata = SignedMetadata {
            contract_address: contracts.tree_nft_contract_index,
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

    /// Inserts a new TreeNftMetadata record in the database.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `req`: The request body containing the metadata to be inserted.
    ///
    /// # Returns
    /// The newly inserted `TreeNftMetadata` record.
    #[oai(
        path = "/admin/tree_nft/metadata",
        method = "post",
        tag = "ApiTags::TreeNftMetadata"
    )]
    pub async fn admin_tree_nft_metadata_insert(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(req): Json<AddMetadataRequest>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let tree_nft_metdata: TreeNftMetadata = req.try_into()?;
        let metadata = tree_nft_metdata
            .insert(&mut conn)?
            .ok_or(Error::BadRequest(PlainText(
                "Failed to insert metadata".to_string(),
            )))?;
        Ok(Json(metadata))
    }

    /// Lists all TreeNftMetadata records in the database, paginated by the given page number.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `page`: The page number to retrieve, starting from 0.
    ///
    /// # Returns
    /// A vector of `TreeNftMetadata` records for the given page.
    #[oai(
        path = "/admin/tree_nft/metadata/list/:page",
        method = "get",
        tag = "ApiTags::TreeNftMetadata"
    )]

    pub async fn admin_tree_nft_metadata_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
    ) -> JsonResult<Vec<TreeNftMetadata>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = TreeNftMetadata::list(&mut conn, PAGE_SIZE, page)?;
        Ok(Json(metadata.into_iter().collect()))
    }

    /// Retrieves a TreeNftMetadata record from the database by its ID.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `id`: The ID of the TreeNftMetadata record to retrieve.
    ///
    /// # Returns
    /// The requested TreeNftMetadata record, or a NotFound error if the record is not found.
    #[oai(
        path = "/admin/tree_nft/metadata/:id",
        method = "get",
        tag = "ApiTags::TreeNftMetadata"
    )]
    pub async fn admin_tree_nft_metadata_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> JsonResult<TreeNftMetadata> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = TreeNftMetadata::find(&mut conn, &id)?
            .ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;

        Ok(Json(metadata))
    }

    /// Deletes a TreeNftMetadata record from the database by its ID.
    ///
    /// This endpoint is only accessible to administrators.
    ///
    /// # Arguments
    /// - `claims`: The authenticated user's claims, used to ensure the user is an admin.
    /// - `db_pool`: A reference to the database connection pool.
    /// - `id`: The ID of the TreeNftMetadata record to delete.
    ///
    /// # Returns
    /// A NoResResult indicating success or failure of the deletion.
    #[oai(
        path = "/admin/tree_nft/metadata/:id",
        method = "delete",
        tag = "ApiTags::TreeNftMetadata"
    )]
    pub async fn admin_tree_nft_metadata_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<String>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let row_count = TreeNftMetadata::delete(&mut conn, &id)?;
        if row_count != 1 {
            return Err(Error::NotFound(PlainText("Metadata not found".to_string())));
        }
        Ok(())
    }

    /// Lists the owners of the NFT with the given metadata ID for the specified contract.
    ///
    /// This endpoint is only accessible to admin users.
    ///
    /// # Parameters
    /// - `claims`: The authenticated user's claims.
    /// - `db_pool`: The database connection pool.
    /// - `contract_index`: The index of the contract to list owners for.
    /// - `metadata_id`: The ID of the metadata to list owners for.
    /// - `page`: The page number to retrieve (optional).
    ///
    /// # Returns
    /// A paged response containing the list of token holders for the specified metadata.
    #[oai(
        path = "/admin/tree_nft/metadata/:metadata_id/owners/:page",
        method = "get",
        tag = "ApiTags::TreeNftMetadata"
    )]
    pub async fn admin_tree_nft_metadata_owners_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(metadata_id): Path<String>,
        Query(page): Query<i64>,
    ) -> JsonResult<PagedResponse<TokenHolder>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let metadata = TreeNftMetadata::find(&mut conn, &metadata_id)?
            .ok_or(Error::NotFound(PlainText("Metadata not found".to_string())))?;
        let (holders, page_count) = list_holders_by_token_metadata_url(
            &mut conn,
            contracts.tree_nft_contract_index,
            &metadata.metadata_url,
            PAGE_SIZE,
            page,
        )?;
        Ok(Json(PagedResponse::into_new(holders, page, page_count)))
    }
}

#[derive(Object, Debug)]
pub struct MetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
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

    pub fn metadata_url(&self) -> Result<::nft_multi_rewarded::MetadataUrl> {
        Ok(::nft_multi_rewarded::MetadataUrl {
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

impl TryInto<TreeNftMetadata> for AddMetadataRequest {
    type Error = Error;

    fn try_into(self) -> Result<TreeNftMetadata> {
        let metadata_url = self.metadata_url()?;

        Ok(TreeNftMetadata::new(
            metadata_url.url,
            metadata_url.hash.map(hex::encode),
            self.probablity_percentage()?,
            chrono::Utc::now(),
        ))
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
    pub contract_address: Decimal,
    pub metadata_url:     MetadataUrl,
    pub account:          String,
    pub account_nonce:    u64,
}

impl SignedMetadata {
    pub fn hash<T>(&self, hasher: T) -> std::result::Result<[u8; 32], HashError>
    where T: FnOnce(Vec<u8>) -> [u8; 32] {
        let internal = ::nft_multi_rewarded::SignedMetadata {
            contract_address: ContractAddress::new(
                self.contract_address
                    .to_u64()
                    .expect("unable to convert contract address to u64"),
                0,
            ),
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
