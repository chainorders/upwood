use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::db_shared::DbConn;
use crate::schema::{
    security_sft_rewards_claimed_reward, security_sft_rewards_contract_rewards,
    security_sft_rewards_reward_tokens as reward_tokens,
};

#[derive(
    Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, AsChangeset,
)]
#[diesel(table_name = security_sft_rewards_contract_rewards)]
#[diesel(primary_key(contract_address, rewarded_token_contract, rewarded_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContractReward {
    pub contract_address:        Decimal,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id:       Decimal,
    pub reward_amount:           Decimal,
    pub create_time:             NaiveDateTime,
    pub update_time:             NaiveDateTime,
}

impl ContractReward {
    pub fn upsert_add_amount(&self, conn: &mut DbConn) -> QueryResult<usize> {
        diesel::insert_into(security_sft_rewards_contract_rewards::table)
            .values(self)
            .on_conflict((
                security_sft_rewards_contract_rewards::contract_address,
                security_sft_rewards_contract_rewards::rewarded_token_contract,
                security_sft_rewards_contract_rewards::rewarded_token_id,
            ))
            .do_update()
            .set((
                security_sft_rewards_contract_rewards::reward_amount
                    .eq(security_sft_rewards_contract_rewards::reward_amount + self.reward_amount),
                security_sft_rewards_contract_rewards::update_time.eq(self.update_time),
            ))
            .execute(conn)
    }

    pub fn sub_amount(
        conn: &mut DbConn,
        contract_address: Decimal,
        rewarded_token_contract: Decimal,
        rewarded_token_id: Decimal,
        reward_amount: Decimal,
        now: NaiveDateTime,
    ) -> QueryResult<usize> {
        diesel::update(
            security_sft_rewards_contract_rewards::table.filter(
                security_sft_rewards_contract_rewards::contract_address
                    .eq(contract_address)
                    .and(
                        security_sft_rewards_contract_rewards::rewarded_token_contract
                            .eq(rewarded_token_contract),
                    )
                    .and(
                        security_sft_rewards_contract_rewards::rewarded_token_id
                            .eq(rewarded_token_id),
                    ),
            ),
        )
        .set((
            security_sft_rewards_contract_rewards::reward_amount
                .eq(security_sft_rewards_contract_rewards::reward_amount - reward_amount),
            security_sft_rewards_contract_rewards::update_time.eq(now),
        ))
        .execute(conn)
    }
}

#[derive(
    Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, AsChangeset,
)]
#[diesel(table_name = reward_tokens)]
#[diesel(primary_key(contract_address, token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RewardToken {
    pub contract_address:        Decimal,
    pub token_id:                Decimal,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id:       Decimal,
    pub reward_amount:           Decimal,
    pub reward_rate:             Decimal,
    pub create_time:             NaiveDateTime,
    pub update_time:             NaiveDateTime,
}

impl RewardToken {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<usize> {
        diesel::insert_into(reward_tokens::table)
            .values(self)
            .execute(conn)
    }

    pub fn sub_amount(
        conn: &mut DbConn,
        contract_address: Decimal,
        token_id: Decimal,
        rewarded_token_contract: Decimal,
        rewarded_token_id: Decimal,
        reward_amount: Decimal,
        now: NaiveDateTime,
    ) -> QueryResult<usize> {
        diesel::update(
            reward_tokens::table.filter(
                reward_tokens::contract_address
                    .eq(contract_address)
                    .and(reward_tokens::token_id.eq(token_id))
                    .and(
                        reward_tokens::rewarded_token_contract
                            .eq(rewarded_token_contract)
                            .and(reward_tokens::rewarded_token_id.eq(rewarded_token_id)),
                    ),
            ),
        )
        .set((
            reward_tokens::reward_amount.eq(reward_tokens::reward_amount - reward_amount),
            reward_tokens::update_time.eq(now),
        ))
        .execute(conn)
    }
}

#[derive(
    Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, AsChangeset,
)]
#[diesel(table_name = security_sft_rewards_claimed_reward)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RewardClaimed {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub holder_address: String,
    pub token_amount: Decimal,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id: Decimal,
    pub reward_amount: Decimal,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl RewardClaimed {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<usize> {
        diesel::insert_into(security_sft_rewards_claimed_reward::table)
            .values(self)
            .execute(conn)
    }
}
