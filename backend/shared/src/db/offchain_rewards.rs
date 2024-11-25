use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::db_shared::DbConn;
use crate::schema::{
    offchain_reward_claims, offchain_reward_contract_agents, offchain_rewardees,
    offchain_rewards_contracts,
};

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    serde::Serialize,
    serde::Deserialize,
    Clone,
)]
#[diesel(table_name = offchain_rewards_contracts)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OffchainRewardsContact {
    pub contract_address: Decimal,
    pub treasury_address: String,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl OffchainRewardsContact {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<OffchainRewardsContact> {
        let ret = diesel::insert_into(offchain_rewards_contracts::table)
            .values(self)
            .returning(OffchainRewardsContact::as_select())
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> QueryResult<Option<Self>> {
        let ret = offchain_rewards_contracts::table
            .filter(offchain_rewards_contracts::contract_address.eq(contract_address))
            .first(conn)
            .optional()?;
        Ok(ret)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    serde::Serialize,
    serde::Deserialize,
    Clone,
)]
#[diesel(table_name = offchain_reward_contract_agents)]
#[diesel(primary_key(contract_address, agent_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OffchainRewardsContractAgent {
    pub contract_address: Decimal,
    pub agent_address:    String,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl OffchainRewardsContractAgent {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<OffchainRewardsContractAgent> {
        let ret = diesel::insert_into(offchain_reward_contract_agents::table)
            .values(self)
            .returning(OffchainRewardsContractAgent::as_select())
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        agent_address: String,
    ) -> QueryResult<Option<Self>> {
        let ret = offchain_reward_contract_agents::table
            .filter(offchain_reward_contract_agents::contract_address.eq(contract_address))
            .filter(offchain_reward_contract_agents::agent_address.eq(agent_address))
            .first(conn)
            .optional()?;
        Ok(ret)
    }

    pub fn delete(&self, conn: &mut DbConn) -> QueryResult<usize> {
        let ret = diesel::delete(
            offchain_reward_contract_agents::table
                .filter(offchain_reward_contract_agents::contract_address.eq(self.contract_address))
                .filter(offchain_reward_contract_agents::agent_address.eq(&self.agent_address)),
        )
        .execute(conn)?;
        Ok(ret)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    AsChangeset,
)]
#[diesel(table_name = offchain_rewardees)]
#[diesel(primary_key(contract_address, account_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OffchainRewardee {
    pub contract_address: Decimal,
    pub account_address:  String,
    pub nonce:            Decimal,
    pub create_time:      NaiveDateTime,
    pub update_time:      NaiveDateTime,
}

impl OffchainRewardee {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<OffchainRewardee> {
        let ret = diesel::insert_into(offchain_rewardees::table)
            .values(self)
            .returning(OffchainRewardee::as_select())
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
        account_address: &str,
    ) -> QueryResult<Option<Self>> {
        let ret = offchain_rewardees::table
            .filter(offchain_rewardees::contract_address.eq(contract_address))
            .filter(offchain_rewardees::account_address.eq(account_address))
            .first(conn)
            .optional()?;
        Ok(ret)
    }

    pub fn update(&self, conn: &mut DbConn) -> QueryResult<OffchainRewardee> {
        let ret = diesel::update(
            offchain_rewardees::table
                .filter(offchain_rewardees::contract_address.eq(self.contract_address))
                .filter(offchain_rewardees::account_address.eq(&self.account_address)),
        )
        .set(self)
        .returning(OffchainRewardee::as_select())
        .get_result(conn)?;
        Ok(ret)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    serde::Serialize,
    serde::Deserialize,
    Clone,
)]
#[diesel(table_name = offchain_reward_claims)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OffchainRewardClaim {
    pub id: Uuid,
    pub block_height: Decimal,
    pub txn_index: Decimal,
    pub contract_address: Decimal,
    pub reward_id: Vec<u8>,
    pub account_address: String,
    pub nonce: Decimal,
    pub reward_amount: Decimal,
    pub reward_token_id: Decimal,
    pub reward_token_contract_address: Decimal,
    pub create_time: NaiveDateTime,
}

impl OffchainRewardClaim {
    pub fn insert(&self, conn: &mut DbConn) -> QueryResult<OffchainRewardClaim> {
        let ret = diesel::insert_into(offchain_reward_claims::table)
            .values(self)
            .returning(OffchainRewardClaim::as_select())
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn find(conn: &mut DbConn, id: Uuid) -> QueryResult<Option<Self>> {
        let ret = offchain_reward_claims::table
            .filter(offchain_reward_claims::id.eq(id))
            .first(conn)
            .optional()?;
        Ok(ret)
    }

    pub fn find_by_reward_id(
        conn: &mut DbConn,
        contract_address: Decimal,
        reward_id: Vec<u8>,
    ) -> QueryResult<Option<Self>> {
        let ret = offchain_reward_claims::table
            .filter(offchain_reward_claims::contract_address.eq(contract_address))
            .filter(offchain_reward_claims::reward_id.eq(reward_id))
            .first(conn)
            .optional()?;
        Ok(ret)
    }
}
