use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use security_sft_single::types::{AgentRole, TokenAmount, TokenId};
use shared::db_shared::DbConn;
use tracing::instrument;

use super::cis2_security;
use crate::processors::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-sft-single/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-single module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_sft_single".to_string())
}

#[instrument(
    name="sft_single",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    now: NaiveDateTime,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    cis2_security::process_events::<TokenId, TokenAmount, AgentRole>(conn, now, contract, events)
}
