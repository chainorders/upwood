use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult};
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db::cis2_security::{
    Agent, Compliance, IdentityRegistry, Token, TokenHolderBalanceUpdateType,
};
use crate::db::txn_listener::{ListenerContract, ProcessorType};
use crate::db_shared::DbConn;
use crate::schema::{cis2_token_holder_balance_updates, cis2_token_holders, users};

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize)]
pub struct TokenContract {
    pub contract_address:    Decimal,
    pub module_ref:          String,
    pub contract_name:       String,
    pub owner:               String,
    pub created_at:          NaiveDateTime,
    pub identity_registry:   Option<Decimal>,
    pub compliance_contract: Option<String>,
    pub agents_count:        i64,
    pub tokens_count:        i64,
}
impl TokenContract {
    pub fn find(conn: &mut DbConn, contract_address: Decimal) -> QueryResult<Option<Self>> {
        let contract = ListenerContract::find(conn, contract_address)?;
        let contract = match contract {
            Some(contract) => {
                if ![
                    ProcessorType::SecuritySftSingle,
                    ProcessorType::SecuritySftMulti,
                    ProcessorType::NftMultiRewarded,
                ]
                .contains(&contract.processor_type)
                {
                    return Ok(None);
                } else {
                    contract
                }
            }
            None => return Ok(None),
        };
        let identity_registry = IdentityRegistry::find(conn, contract_address)?;
        let compliance_contract = Compliance::find(conn, contract_address)?;
        let agents_count = Agent::count(conn, contract_address)?;
        let tokens_count = Token::count(conn, contract_address)?;
        let contract = TokenContract {
            contract_address,
            module_ref: contract.module_ref,
            contract_name: contract.contract_name,
            owner: contract.owner,
            created_at: contract.created_at,
            identity_registry: identity_registry.map(|ir| ir.identity_registry_address),
            compliance_contract: compliance_contract.map(|cc| cc.compliance_address),
            agents_count,
            tokens_count,
        };
        Ok(Some(contract))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize)]
pub struct TokenHolderUser {
    pub cis2_address:      Decimal,
    pub token_id:          Decimal,
    pub holder_address:    String,
    pub frozen_balance:    Decimal,
    pub un_frozen_balance: Decimal,
    pub create_time:       NaiveDateTime,
    pub update_time:       NaiveDateTime,
    pub cognito_user_id:   Option<String>,
    pub email:             Option<String>,
}

impl TokenHolderUser {
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<&str>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = cis2_token_holders::table.left_join(
            users::table.on(cis2_token_holders::holder_address.eq(users::account_address)),
        );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        if let Some(cis2_address) = cis2_address {
            query = query.filter(cis2_token_holders::cis2_address.eq(cis2_address));
            count_query = count_query.filter(cis2_token_holders::cis2_address.eq(cis2_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(cis2_token_holders::token_id.eq(token_id));
            count_query = count_query.filter(cis2_token_holders::token_id.eq(token_id));
        }
        if let Some(holder_address) = holder_address {
            query = query.filter(cis2_token_holders::holder_address.eq(holder_address));
            count_query = count_query.filter(cis2_token_holders::holder_address.eq(holder_address));
        }
        let total_count = count_query
            .select(diesel::dsl::count(cis2_token_holders::holder_address))
            .first::<i64>(conn)?;
        let query = query
            .select((
                cis2_token_holders::cis2_address,
                cis2_token_holders::token_id,
                cis2_token_holders::holder_address,
                cis2_token_holders::frozen_balance,
                cis2_token_holders::un_frozen_balance,
                cis2_token_holders::create_time,
                cis2_token_holders::update_time,
                users::cognito_user_id.nullable(),
                users::email.nullable(),
            ))
            .order(cis2_token_holders::create_time.desc())
            .limit(page_size)
            .offset(page * page_size);
        let records = query.load::<(
            Decimal,
            Decimal,
            String,
            Decimal,
            Decimal,
            NaiveDateTime,
            NaiveDateTime,
            Option<String>,
            Option<String>,
        )>(conn)?;
        let records = records
            .into_iter()
            .map(|record| TokenHolderUser {
                cis2_address:      record.0,
                token_id:          record.1,
                holder_address:    record.2,
                frozen_balance:    record.3,
                un_frozen_balance: record.4,
                create_time:       record.5,
                update_time:       record.6,
                cognito_user_id:   record.7,
                email:             record.8,
            })
            .collect();
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &str,
    ) -> QueryResult<Option<Self>> {
        let query = cis2_token_holders::table
            .left_join(
                users::table.on(cis2_token_holders::holder_address.eq(users::account_address)),
            )
            .filter(cis2_token_holders::cis2_address.eq(cis2_address))
            .filter(cis2_token_holders::token_id.eq(token_id))
            .filter(cis2_token_holders::holder_address.eq(holder_address));
        let record = query
            .select((
                cis2_token_holders::cis2_address,
                cis2_token_holders::token_id,
                cis2_token_holders::holder_address,
                cis2_token_holders::frozen_balance,
                cis2_token_holders::un_frozen_balance,
                cis2_token_holders::create_time,
                cis2_token_holders::update_time,
                users::cognito_user_id.nullable(),
                users::email.nullable(),
            ))
            .first::<(
                Decimal,
                Decimal,
                String,
                Decimal,
                Decimal,
                NaiveDateTime,
                NaiveDateTime,
                Option<String>,
                Option<String>,
            )>(conn)
            .optional()?;
        let record = match record {
            Some(record) => TokenHolderUser {
                cis2_address:      record.0,
                token_id:          record.1,
                holder_address:    record.2,
                frozen_balance:    record.3,
                un_frozen_balance: record.4,
                create_time:       record.5,
                update_time:       record.6,
                cognito_user_id:   record.7,
                email:             record.8,
            },
            None => return Ok(None),
        };
        Ok(Some(record))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Object, Serialize, Deserialize)]
pub struct TokenHolderUserBalanceUpdate {
    pub id:                uuid::Uuid,
    pub block_height:      Decimal,
    pub txn_index:         Decimal,
    pub cis2_address:      Decimal,
    pub token_id:          Decimal,
    pub holder_address:    String,
    pub amount:            Decimal,
    pub frozen_balance:    Decimal,
    pub un_frozen_balance: Decimal,
    pub txn_sender:        String,
    pub txn_instigator:    String,
    pub update_type:       TokenHolderBalanceUpdateType,
    pub create_time:       NaiveDateTime,
    pub cognito_user_id:   Option<String>,
    pub email:             Option<String>,
}

impl TokenHolderUserBalanceUpdate {
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<&str>,
        update_type: Option<TokenHolderBalanceUpdateType>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let query = cis2_token_holder_balance_updates::table.left_join(
            users::table
                .on(cis2_token_holder_balance_updates::holder_address.eq(users::account_address)),
        );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        if let Some(cis2_address) = cis2_address {
            query = query.filter(cis2_token_holder_balance_updates::cis2_address.eq(cis2_address));
            count_query = count_query
                .filter(cis2_token_holder_balance_updates::cis2_address.eq(cis2_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(cis2_token_holder_balance_updates::token_id.eq(token_id));
            count_query =
                count_query.filter(cis2_token_holder_balance_updates::token_id.eq(token_id));
        }
        if let Some(holder_address) = holder_address {
            query =
                query.filter(cis2_token_holder_balance_updates::holder_address.eq(holder_address));
            count_query = count_query
                .filter(cis2_token_holder_balance_updates::holder_address.eq(holder_address));
        }
        if let Some(update_type) = update_type {
            query = query.filter(cis2_token_holder_balance_updates::update_type.eq(update_type));
            count_query =
                count_query.filter(cis2_token_holder_balance_updates::update_type.eq(update_type));
        }
        let total_count = count_query
            .select(diesel::dsl::count(cis2_token_holder_balance_updates::id))
            .first::<i64>(conn)?;
        let query = query
            .select((
                cis2_token_holder_balance_updates::id,
                cis2_token_holder_balance_updates::block_height,
                cis2_token_holder_balance_updates::txn_index,
                cis2_token_holder_balance_updates::cis2_address,
                cis2_token_holder_balance_updates::token_id,
                cis2_token_holder_balance_updates::holder_address,
                cis2_token_holder_balance_updates::amount,
                cis2_token_holder_balance_updates::frozen_balance,
                cis2_token_holder_balance_updates::un_frozen_balance,
                cis2_token_holder_balance_updates::txn_sender,
                cis2_token_holder_balance_updates::txn_instigator,
                cis2_token_holder_balance_updates::update_type,
                cis2_token_holder_balance_updates::create_time,
                users::cognito_user_id.nullable(),
                users::email.nullable(),
            ))
            .order(cis2_token_holder_balance_updates::create_time.desc())
            .limit(page_size)
            .offset(page * page_size);
        let records = query.load::<(
            uuid::Uuid,
            Decimal,
            Decimal,
            Decimal,
            Decimal,
            String,
            Decimal,
            Decimal,
            Decimal,
            String,
            String,
            TokenHolderBalanceUpdateType,
            NaiveDateTime,
            Option<String>,
            Option<String>,
        )>(conn)?;
        let records = records
            .into_iter()
            .map(|record| TokenHolderUserBalanceUpdate {
                id:                record.0,
                block_height:      record.1,
                txn_index:         record.2,
                cis2_address:      record.3,
                token_id:          record.4,
                holder_address:    record.5,
                amount:            record.6,
                frozen_balance:    record.7,
                un_frozen_balance: record.8,
                txn_sender:        record.9,
                txn_instigator:    record.10,
                update_type:       record.11,
                create_time:       record.12,
                cognito_user_id:   record.13,
                email:             record.14,
            })
            .collect();
        let page_count = if total_count == 0 {
            0
        } else {
            (total_count as f64 / page_size as f64).ceil() as i64
        };
        Ok((records, page_count))
    }
}
