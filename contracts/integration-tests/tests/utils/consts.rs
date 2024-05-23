use concordium_std::Amount;
pub const IDENTITY_REGISTRY_MODULE: &str = "../identity-registry/contract.wasm.v1";
pub const COMPLIANCE_MODULE: &str = "../compliance/contract.wasm.v1";
pub const SPONSOR_MODULE: &str = "../sponsor/contract.wasm.v1";
pub const SECURITY_NFT_MODULE: &str = "../security-nft/contract.wasm.v1";
pub const SECURITY_SFT_MODULE: &str = "../security-sft/contract.wasm.v1";
pub const MARKET_MODULE: &str = "../market/contract.wasm.v1";
pub const EUROE_MODULE: &str = "../euroe/dist/module.wasm.v1";
pub const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};
