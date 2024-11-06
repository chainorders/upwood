use chrono::{DateTime, Utc};
use concordium_rust_sdk::types::Address;
use diesel::dsl::*;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use shared::db::{DbConn, DbResult};

use crate::schema::{cis2_tokens, nft_multi_address_nonces, nft_multi_rewarded_contracts};

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    serde::Serialize,
    Clone,
)]
#[diesel(table_name = nft_multi_rewarded_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NftMultiRewardedContract {
    pub contract_address:     Decimal,
    pub reward_token_address: Decimal,
    pub reward_token_id:      Decimal,
    pub create_time:          chrono::NaiveDateTime,
    pub update_time:          chrono::NaiveDateTime,
}

impl NftMultiRewardedContract {
    pub fn new(
        contract: Decimal,
        reward_token_address: Decimal,
        reward_token_id: Decimal,
        block_slot_time: DateTime<Utc>,
    ) -> Self {
        Self {
            contract_address: contract,
            reward_token_address,
            reward_token_id,
            create_time: block_slot_time.naive_utc(),
            update_time: block_slot_time.naive_utc(),
        }
    }

    pub fn find(
        conn: &mut DbConn,
        contract: Decimal,
    ) -> DbResult<Option<NftMultiRewardedContract>> {
        let contract = nft_multi_rewarded_contracts::table
            .filter(nft_multi_rewarded_contracts::contract_address.eq(contract))
            .first(conn)
            .optional()?;
        Ok(contract)
    }

    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(nft_multi_rewarded_contracts::table)
            .values(self)
            .on_conflict(nft_multi_rewarded_contracts::contract_address)
            .do_update()
            .set((
                nft_multi_rewarded_contracts::reward_token_address.eq(&self.reward_token_address),
                nft_multi_rewarded_contracts::reward_token_id.eq(&self.reward_token_id),
                nft_multi_rewarded_contracts::update_time.eq(self.update_time),
            ))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = nft_multi_address_nonces)]
#[diesel(primary_key(contract_address, address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AddressNonce {
    pub contract_address: Decimal,
    pub address:          String,
    pub nonce:            i64,
}

impl AddressNonce {
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(nft_multi_address_nonces::table)
            .values(self)
            .on_conflict((
                nft_multi_address_nonces::contract_address,
                nft_multi_address_nonces::address,
            ))
            .do_update()
            .set(nft_multi_address_nonces::nonce.eq(&self.nonce))
            .execute(conn)?;
        Ok(())
    }

    pub fn find(
        conn: &mut DbConn,
        contract: Decimal,
        address: &Address,
    ) -> DbResult<Option<AddressNonce>> {
        let nonce = nft_multi_address_nonces::table
            .filter(nft_multi_address_nonces::contract_address.eq(contract))
            .filter(nft_multi_address_nonces::address.eq(address.to_string()))
            .first(conn)
            .optional()?;
        Ok(nonce)
    }
}

#[derive(Debug, Clone, serde::Serialize, Object)]
pub struct NftMultiRewardedDetails {
    pub contract:             NftMultiRewardedContract,
    pub tokens_count:         u64,
    pub unique_metdata_count: u64,
}

impl NftMultiRewardedDetails {
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
    ) -> DbResult<Option<NftMultiRewardedDetails>> {
        let res = nft_multi_rewarded_contracts::table
            .inner_join(
                cis2_tokens::table.on(nft_multi_rewarded_contracts::reward_token_address
                    .eq(cis2_tokens::cis2_address)
                    .and(nft_multi_rewarded_contracts::reward_token_id.eq(cis2_tokens::token_id))),
            )
            .group_by(nft_multi_rewarded_contracts::contract_address)
            .filter(nft_multi_rewarded_contracts::contract_address.eq(contract_address))
            .select((
                NftMultiRewardedContract::as_select(),
                count_distinct(cis2_tokens::token_id),
                count_distinct(cis2_tokens::metadata_url),
            ))
            .first::<(NftMultiRewardedContract, i64, i64)>(conn)
            .optional()?;

        let (contract, tokens_count, unique_metdata_count) = match res {
            Some(res) => res,
            None => return Ok(None),
        };
        Ok(Some(NftMultiRewardedDetails {
            contract,
            tokens_count: tokens_count as u64,
            unique_metdata_count: unique_metdata_count as u64,
        }))
    }
}
