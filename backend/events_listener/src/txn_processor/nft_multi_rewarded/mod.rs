use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};

pub mod db;
pub mod processor;
pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../../contracts/nft-multi-rewarded/contract.wasm.v1"
    ))
    .expect("Failed to parse nft-multi-rewarded module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_nft_multi_rewarded".to_string())
}
