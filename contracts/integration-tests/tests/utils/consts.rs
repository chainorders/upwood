use concordium_cis2::{IsTokenAmount, IsTokenId, TokenAmountU64, TokenIdVec};
use concordium_std::{to_bytes, AccountAddress, Amount, Cursor, Deserial, Serial};

pub const IDENTITY_REGISTRY_MODULE: &str = "../identity-registry/contract.wasm.v1";
pub const IR_CONTRACT_NAME: &str = "init_rwa_identity_registry";
pub const COMPLIANCE_MODULE: &str = "../compliance/contract.wasm.v1";
pub const COMPLIANCE_CONTRACT_NAME: &str = "init_rwa_compliance";
pub const COMPLIANCE_MODULE_CONTRACT_NAME: &str =
    "init_rwa_compliance_module_allowed_nationalities";
pub const SECURITY_NFT_MODULE: &str = "../security-nft/contract.wasm.v1";
pub const SECURITY_SFT_MODULE: &str = "../security-sft/contract.wasm.v1";
pub const SECURITY_NFT_CONTRACT_NAME: &str = "init_rwa_security_nft";
pub const SECURITY_SFT_CONTRACT_NAME: &str = "init_rwa_security_sft";
pub const MARKET_MODULE: &str = "../market/contract.wasm.v1";
pub const MARKET_CONTRACT_NAME: &str = "init_rwa_market";
pub const EUROE_MODULE: &str = "../euroe/dist/module.wasm.v1";
pub const EUROE_CONTRACT_NAME: &str = "init_euroe_stablecoin";
pub const DEFAULT_INVOKER: AccountAddress = AccountAddress([255; 32]);
pub const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;
pub const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};
pub fn to_token_id_vec<T: IsTokenId + Serial>(token_id: T) -> TokenIdVec {
    TokenIdVec(to_bytes(&token_id)[1..].to_vec())
}

pub fn to_token_amount_u64<A: IsTokenAmount + Serial>(token_amount: A) -> TokenAmountU64 {
    let mut token_amount_bytes = to_bytes(&token_amount);
    let mut cursor = Cursor::new(&mut token_amount_bytes);
    TokenAmountU64::deserial(&mut cursor).unwrap()
}
