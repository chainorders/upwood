use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::{Address, ContractAddress};
use diesel::prelude::*;
use nft_multi_rewarded::types::RewardTokenId;
use shared::db::{DbConn, DbResult};

use crate::schema::{nft_multi_address_nonces, nft_multi_rewarded_contracts};

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

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = nft_multi_address_nonces)]
#[diesel(primary_key(contract_address, address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AddressNonce {
    pub contract_address: String,
    pub address:          String,
    pub nonce:            i64,
}

pub fn upsert_address_nonce(conn: &mut DbConn, record: &AddressNonce) -> DbResult<()> {
    diesel::insert_into(nft_multi_address_nonces::table)
        .values(record)
        .on_conflict((
            nft_multi_address_nonces::contract_address,
            nft_multi_address_nonces::address,
        ))
        .do_update()
        .set(nft_multi_address_nonces::nonce.eq(record.nonce))
        .execute(conn)?;
    Ok(())
}

pub fn find_address_nonce(
    conn: &mut DbConn,
    contract: &ContractAddress,
    address: &Address,
) -> DbResult<Option<i64>> {
    let nonce = nft_multi_address_nonces::table
        .filter(nft_multi_address_nonces::contract_address.eq(contract.to_string()))
        .filter(nft_multi_address_nonces::address.eq(address.to_string()))
        .select(nft_multi_address_nonces::nonce)
        .first(conn)
        .optional()?;
    Ok(nonce)
}
