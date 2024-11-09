use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};

pub mod processor;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../../contracts/security-sft-rewards/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-rewards module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_sft_rewards".to_string())
}
