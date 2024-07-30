use super::error::Error;
use concordium_cis2::{OnReceivingCis2Params, Receiver, TokenIdVec};
use concordium_rwa_utils::{cis2_types, concordium_cis2_security};
use concordium_std::*;

pub type ContractResult<R> = Result<R, Error>;
pub type TokenAmount = cis2_types::SftTokenAmount;
pub type TokenId = cis2_types::SftTokenId;
pub type NftTokenAmount = cis2_types::NftTokenAmount;
pub type NftTokenId = TokenIdVec;
pub type NftTokenUId = concordium_cis2_security::TokenUId<NftTokenId>;
pub type NftTokenOwnerUId = concordium_cis2_security::TokenOwnerUId<NftTokenId>;
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
pub type Event = concordium_cis2_security::Cis2SecurityEvent<TokenId, TokenAmount>;
pub use cis2_types::ContractMetadataUrl;
pub use concordium_cis2_security::RecoverParam;
pub use concordium_rwa_utils::cis2_conversions::Rate;

#[derive(Serialize, SchemaType)]
pub struct InitParam {
    pub identity_registry: ContractAddress,
    pub compliance:        ContractAddress,
    pub sponsors:          Vec<ContractAddress>,
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
