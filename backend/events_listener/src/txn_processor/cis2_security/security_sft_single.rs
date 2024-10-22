use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use security_sft_single::types::{AgentRole, TokenAmount, TokenId};
use shared::db::DbConn;
use tracing::instrument;

use crate::txn_listener::listener::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../../contracts/security-sft-single/contract.wasm.v1"
    ))
    .expect("Failed to parse security-sft-single module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_sft_single".to_string())
}

#[instrument(
    name="security_sft_single_process_events",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    super::processor::process_events::<TokenId, TokenAmount, AgentRole>(conn, now, contract, events)
}
