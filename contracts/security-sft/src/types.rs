use super::error::Error;
use concordium_cis2::{OnReceivingCis2Params, Receiver, TokenIdVec};
pub use concordium_rwa_utils::cis2_conversions::Rate;
use concordium_rwa_utils::{cis2_schema_types, cis2_types, concordium_cis2_security};
use concordium_std::*;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = cis2_types::SftTokenAmount;
pub type TokenId = cis2_types::SftTokenId;
pub type NftTokenAmount = cis2_types::NftTokenAmount;
pub type NftTokenId = TokenIdVec;
pub type NftTokenUId = cis2_schema_types::TokenUId<NftTokenId>;
pub type NftTokenOwnerUId = cis2_schema_types::TokenOwnerUId<NftTokenId>;
pub type ContractTransferParams = concordium_cis2::TransferParams<TokenId, TokenAmount>;
pub type ContractBalanceOfQueryParams = concordium_cis2::BalanceOfQueryParams<TokenId>;
pub type ContractBalanceOfQuery = concordium_cis2::BalanceOfQuery<TokenId>;
pub type ContractBalanceOfQueryResponse = concordium_cis2::BalanceOfQueryResponse<TokenAmount>;
pub type PauseParams = concordium_cis2_security::PauseParams<TokenId>;
pub type IsPausedResponse = concordium_cis2_security::IsPausedResponse;
pub type BurnParams = concordium_cis2_security::BurnParams<TokenId, TokenAmount>;
pub type Burn = concordium_cis2_security::Burn<TokenId, TokenAmount>;
pub type FreezeParams = concordium_cis2_security::FreezeParams<TokenId, TokenAmount>;
pub type FrozenParams = concordium_cis2_security::FrozenParams<TokenId>;
pub type FrozenResponse = concordium_cis2_security::FrozenResponse<TokenAmount>;
pub type DepositParams = OnReceivingCis2Params<NftTokenId, NftTokenAmount>;
pub use concordium_cis2_security::RecoverParam;

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub identity_registry: ContractAddress,
    pub compliance:        ContractAddress,
    pub sponsors:          Vec<ContractAddress>,
}

/// Represents the metadata URL and hash of a token.
#[derive(SchemaType, Serial, Clone, Deserial)]
pub struct ContractMetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

impl From<ContractMetadataUrl> for MetadataUrl {
    fn from(val: ContractMetadataUrl) -> Self {
        MetadataUrl {
            url:  val.url,
            hash: {
                if let Some(hash) = val.hash {
                    let mut hash_bytes = [0u8; 32];
                    match hex::decode_to_slice(hash, &mut hash_bytes) {
                        Ok(_) => Some(hash_bytes),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
        }
    }
}

#[derive(Serialize, SchemaType)]
pub struct AddParam {
    pub deposit_token_id: NftTokenUId,
    pub metadata_url:     ContractMetadataUrl,
    pub fractions_rate:   Rate,
}

#[derive(Serialize, SchemaType)]
pub struct AddParams {
    pub tokens: Vec<AddParam>,
}

#[derive(Serialize, SchemaType)]
pub struct MintParam {
    /// The token id of the deposited token.
    pub deposited_token_id:    NftTokenUId,
    /// The owner of the deposited token.
    pub deposited_token_owner: AccountAddress,
    /// The amount of the deposited token.
    pub deposited_amount:      NftTokenAmount,
    /// The owner of the minted token.
    pub owner:                 Receiver,
}

#[derive(Serialize, SchemaType, Clone)]
pub struct WithdrawParams {
    pub token_id: NftTokenUId,
    pub owner:    AccountAddress,
    pub amount:   NftTokenAmount,
}

#[derive(Serialize, SchemaType)]
pub struct BalanceOfDepositParams {
    pub token_id: NftTokenUId,
    pub address:  AccountAddress,
}
