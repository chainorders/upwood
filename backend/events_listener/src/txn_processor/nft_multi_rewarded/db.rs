use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::ContractAddress;
use concordium_rwa_backend_shared::db::{DbConn, DbResult};
use diesel::prelude::*;
use nft_multi_rewarded::types::RewardTokenId;

use crate::schema::nft_multi_rewarded_contracts;

pub fn upsert_reward_token(
    conn: &mut DbConn,
    now: DateTime<Utc>,
    contract: &ContractAddress,
    reward_token_contract: &ContractAddress,
    reward_token_id: &RewardTokenId,
) -> DbResult<()> {
    diesel::insert_into(nft_multi_rewarded_contracts::table)
        .values((
            nft_multi_rewarded_contracts::contract_address.eq(contract.to_string()),
            nft_multi_rewarded_contracts::reward_token_address
                .eq(reward_token_contract.to_string()),
            nft_multi_rewarded_contracts::reward_token_id.eq(reward_token_id.to_string()),
            nft_multi_rewarded_contracts::update_time.eq(now.naive_utc()),
        ))
        .on_conflict(nft_multi_rewarded_contracts::contract_address)
        .do_update()
        .set((
            nft_multi_rewarded_contracts::reward_token_address
                .eq(reward_token_contract.to_string()),
            nft_multi_rewarded_contracts::reward_token_id.eq(reward_token_id.to_string()),
            nft_multi_rewarded_contracts::update_time.eq(now.naive_utc()),
        ))
        .execute(conn)
        .map_err(Into::into)
        .map(|_| ())
}
