use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use shared::db::{DbConn, DbResult};

use crate::txn_listener::db::ListenerContract;
use crate::txn_processor::cis2_security::db::Token;
use crate::txn_processor::cis2_utils::TokenIdToDecimal;

#[derive(Object)]
pub struct ContractReward {
    pub contract_address:          Decimal,
    pub token_id:                  Decimal,
    pub rewarded_contract_address: Decimal,
    pub rewarded_token_id:         Decimal,
    pub reward_amount:             Decimal,
    pub reward_rate_numerator:     i64,
    pub reward_rate_denominator:   i64,
    pub created_at:                NaiveDateTime,
    pub updated_at:                NaiveDateTime,
}

#[derive(Object)]
pub struct SecuritySftRewardsContract {
    pub contract:         ListenerContract,
    pub tracked_token_id: Decimal,
}

impl SecuritySftRewardsContract {
    pub fn tracked_token_id() -> Decimal {
        security_sft_rewards::types::TRACKED_TOKEN_ID.to_decimal()
    }

    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> DbResult<Option<Self>> {
        let contract = match ListenerContract::find(conn, contract_address)? {
            Some(contract) => contract,
            None => return Ok(None),
        };

        Ok(Some(Self {
            contract,
            tracked_token_id: Self::tracked_token_id(),
        }))
    }

    pub fn token_tracked(&self, conn: &mut DbConn) -> DbResult<Option<Token>> {
        Token::find(
            conn,
            self.contract.contract_address,
            Self::tracked_token_id(),
        )
    }
}

diesel::table! {
    reward_holder_aggregates (contract_address, holder_address, token_id, rewarded_contract_address, rewarded_token_id) {
        contract_address -> Numeric,
        holder_address -> Varchar,
        token_id -> Numeric,
        rewarded_contract_address -> Numeric,
        rewarded_token_id -> Numeric,
        rewards_frozen -> Numeric,
        rewards_un_frozen -> Numeric,
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Object)]
#[diesel(table_name = reward_holder_aggregates)]
#[diesel(primary_key(contract_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RewardHolderAggregate {
    pub contract_address:          Decimal,
    pub holder_address:            String,
    pub token_id:                  Decimal,
    pub rewarded_contract_address: Decimal,
    pub rewarded_token_id:         Decimal,
    pub rewards_frozen:            Decimal,
    pub rewards_un_frozen:         Decimal,
}

#[derive(Object)]
pub struct RewardHolderTotal {
    pub holder_address:            String,
    pub rewarded_contract_address: Decimal,
    pub rewarded_token_id:         Decimal,
    pub rewards_frozen:            Decimal,
    pub rewards_un_frozen:         Decimal,
}

impl RewardHolderTotal {
    pub fn find(
        conn: &mut DbConn,
        contract_addresses: Vec<Decimal>,
        holder_address: &str,
    ) -> DbResult<Vec<Self>> {
        let res = reward_holder_aggregates::table
            .group_by((
                reward_holder_aggregates::holder_address,
                reward_holder_aggregates::rewarded_contract_address,
                reward_holder_aggregates::rewarded_token_id,
            ))
            .filter(reward_holder_aggregates::contract_address.eq_any(contract_addresses))
            .filter(reward_holder_aggregates::holder_address.eq(holder_address))
            .select((
                reward_holder_aggregates::holder_address,
                reward_holder_aggregates::rewarded_contract_address,
                reward_holder_aggregates::rewarded_token_id,
                sum(reward_holder_aggregates::rewards_frozen),
                sum(reward_holder_aggregates::rewards_un_frozen),
            ))
            .load::<(String, Decimal, Decimal, Option<Decimal>, Option<Decimal>)>(conn)?;

        let res = res
            .into_iter()
            .map(
                |(
                    holder_address,
                    rewarded_contract_address,
                    rewarded_token_id,
                    rewards_frozen,
                    rewards_un_frozen,
                )| Self {
                    holder_address,
                    rewarded_contract_address,
                    rewarded_token_id,
                    rewards_frozen: rewards_frozen.unwrap_or_default(),
                    rewards_un_frozen: rewards_un_frozen.unwrap_or_default(),
                },
            )
            .collect();

        Ok(res)
    }
}

diesel::table! {
    reward_holder (contract_address, holder_address, token_id, rewarded_contract_address, rewarded_token_id) {
        contract_address -> Numeric,
        token_id -> Numeric,
        holder_address -> Varchar,
        frozen_balance -> Numeric,
        un_frozen_balance -> Numeric,
        rewarded_contract_address -> Numeric,
        rewarded_token_id -> Numeric,
        rewards_frozen -> Numeric,
        rewards_un_frozen -> Numeric,
        rewards_rate_numerator -> Int8,
        rewards_rate_denominator -> Int8,
    }
}

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Object)]
#[diesel(table_name = reward_holder)]
#[diesel(primary_key(
    contract_address,
    holder_address,
    token_id,
    rewarded_contract_address,
    rewarded_token_id
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RewardHolder {
    pub contract_address:          Decimal,
    pub token_id:                  Decimal,
    pub holder_address:            String,
    pub frozen_balance:            Decimal,
    pub un_frozen_balance:         Decimal,
    pub rewarded_contract_address: Decimal,
    pub rewarded_token_id:         Decimal,
    pub rewards_frozen:            Decimal,
    pub rewards_un_frozen:         Decimal,
    pub rewards_rate_numerator:    i64,
    pub rewards_rate_denominator:  i64,
}

impl RewardHolder {
    pub fn list(
        conn: &mut DbConn,
        contract_addresses: Vec<Decimal>,
        holder_address: &str,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let query = reward_holder::table
            .filter(reward_holder::contract_address.eq_any(contract_addresses))
            .filter(reward_holder::holder_address.eq(holder_address))
            .select(RewardHolder::as_select());
        let res = query
            .clone()
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let count = query.count().get_result::<i64>(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((res, page_count))
    }
}
