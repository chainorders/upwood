use chrono::NaiveDateTime;
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_sft_single::types::{AgentRole, TokenAmount, TokenId};
use shared::db_shared::DbConn;
use tracing::instrument;

use super::cis2_security;
use crate::processors::ProcessorError;
#[instrument(
    name="sft_multi",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    cis2_security::process_events::<TokenId, TokenAmount, AgentRole>(
        conn,
        block_height,
        block_time,
        txn_index,
        contract,
        events,
    )
}
