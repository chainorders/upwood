use chrono::{DateTime, Utc};
use concordium_cis2::TokenIdUnit;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use poem_openapi::Object;
use rust_decimal::Decimal;
use security_sft_single::types::{AgentRole, TokenAmount, TokenId};
use shared::db::{to_u64, DbConn, DbResult};
use tracing::instrument;

use crate::txn_listener;
use crate::txn_listener::db::ListenerContract;
use crate::txn_listener::listener::ProcessorError;
use crate::txn_processor::cis2_security::{self, db};
use crate::txn_processor::cis2_utils::TokenIdToDecimal;

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

#[derive(serde::Serialize, Object)]
pub struct TokenDetails {
    pub supply:          u64,
    pub holder_count:    u64,
    pub token_id:        String,
    pub contract_agents: Vec<db::Agent>,
    pub contract:        txn_listener::db::ListenerContract,
}

impl TokenDetails {
    pub fn find(contract: Decimal, db_conn: &mut DbConn) -> DbResult<Option<TokenDetails>> {
        let token_id = TokenIdUnit().to_decimal();
        let contract = ListenerContract::find(db_conn, contract)?;
        let contract = match contract {
            Some(contract_details) => contract_details,
            None => return Ok(None),
        };
        let token_details =
            cis2_security::db::Token::find(db_conn, contract.contract_address, token_id)?;
        let token_details = match token_details {
            None => return Ok(None),
            Some(td) => td,
        };
        let (contract_agents, _) =
            cis2_security::db::Agent::list(db_conn, contract.contract_address, i64::MAX, 0)?;
        let holder_count = cis2_security::db::holders_count_by_token(
            db_conn,
            contract.contract_address,
            token_id,
        )?;

        Ok(Some(TokenDetails {
            supply: to_u64(token_details.supply),
            holder_count: holder_count as u64,
            token_id: token_id.to_string(),
            contract_agents,
            contract,
        }))
    }
}
