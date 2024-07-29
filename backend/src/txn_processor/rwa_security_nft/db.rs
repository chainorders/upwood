use std::ops::{Add, Sub};

use crate::{
    schema::{
        security_cis2_contract_agents, security_cis2_contract_compliances,
        security_cis2_contract_identity_registries, security_cis2_contract_operators,
        security_cis2_contract_recovery_records, security_cis2_contract_token_holders,
        security_cis2_contract_tokens,
    },
    shared::db::{token_amount_to_sql, DbConn, DbResult},
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::{
    cis2,
    types::{Address, ContractAddress},
};
use diesel::prelude::*;
use num_traits::Zero;
#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_agents)]
#[diesel(primary_key(cis2_address, agent_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub cis2_address:  String,
    pub agent_address: String,
}

impl Agent {
    pub fn new(
        agent_address: Address,
        _time: DateTime<Utc>,
        cis2_address: &ContractAddress,
    ) -> Self {
        Self {
            agent_address: agent_address.to_string(),
            cis2_address:  cis2_address.to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn list_agents(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<Agent>, i64)> {
    let select_filter = security_cis2_contract_agents::cis2_address.eq(cis2_address.to_string());
    let res: Vec<Agent> = security_cis2_contract_agents::table
        .filter(select_filter.clone())
        .select(Agent::as_select())
        .limit(page_size)
        .offset(page_size * page)
        .get_results(conn)?;
    let count: i64 =
        security_cis2_contract_agents::table.filter(select_filter).count().get_result(conn)?;
    let page_count = (count + page_size - 1) / page_size;

    Ok((res, page_count))
}

pub fn insert_agent(conn: &mut DbConn, agent: Agent) -> DbResult<usize> {
    diesel::insert_into(security_cis2_contract_agents::table).values(agent).execute(conn)
}

pub fn remove_agent(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    agent_address: &Address,
) -> DbResult<usize> {
    let delete_filter = security_cis2_contract_agents::cis2_address
        .eq(cis2_address.to_string())
        .and(security_cis2_contract_agents::agent_address.eq(agent_address.to_string()));
    diesel::delete(security_cis2_contract_agents::table).filter(delete_filter).execute(conn)
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_compliances)]
#[diesel(primary_key(cis2_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2ContractCompliance {
    pub cis2_address:       String,
    pub compliance_address: String,
}

impl SecurityCis2ContractCompliance {
    pub fn new(cis2_address: &ContractAddress, compliance: &ContractAddress) -> Self {
        SecurityCis2ContractCompliance {
            cis2_address:       cis2_address.to_string(),
            compliance_address: compliance.to_string(),
        }
    }
}

pub fn upsert_compliance(
    conn: &mut DbConn,
    record: &SecurityCis2ContractCompliance,
) -> DbResult<()> {
    let row_count = diesel::insert_into(security_cis2_contract_compliances::table)
        .values(record)
        .on_conflict((security_cis2_contract_compliances::cis2_address,))
        .do_update()
        .set(record)
        .execute(conn)?;

    assert_eq!(row_count, 1, "More than one row updated");

    Ok(())
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name =
security_cis2_contract_identity_registries)]
#[diesel(primary_key(cis2_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2ContractIdentityRegistry {
    pub cis2_address:              String,
    pub identity_registry_address: String,
}

impl SecurityCis2ContractIdentityRegistry {
    pub fn new(
        cis2_address: &ContractAddress,
        identity_registry_address: &ContractAddress,
    ) -> Self {
        SecurityCis2ContractIdentityRegistry {
            cis2_address:              cis2_address.to_string(),
            identity_registry_address: identity_registry_address.to_string(),
        }
    }
}

pub fn upsert_identity_registry(
    conn: &mut DbConn,
    record: &SecurityCis2ContractIdentityRegistry,
) -> DbResult<()> {
    let row_count = diesel::insert_into(security_cis2_contract_identity_registries::table)
        .values(record)
        .on_conflict((security_cis2_contract_identity_registries::cis2_address,))
        .do_update()
        .set(record)
        .execute(conn)?;

    assert_eq!(row_count, 1, "More than one row updated");

    Ok(())
}

pub fn update_token_paused(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: &cis2::TokenId,
    paused: bool,
) -> DbResult<()> {
    let update_filter = security_cis2_contract_tokens::cis2_address
        .eq(cis2_address.to_string())
        .and(security_cis2_contract_tokens::token_id.eq(token_id.to_string()));
    let update = security_cis2_contract_tokens::is_paused.eq(paused);

    let row_count = diesel::update(security_cis2_contract_tokens::table)
        .filter(update_filter)
        .set(update)
        .execute(conn)?;
    assert_eq!(row_count, 1, "More than one row updated");

    Ok(())
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_token_holders)]
#[diesel(primary_key(cis2_address, token_id, holder_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2TokenHolder {
    pub cis2_address:   String,
    pub token_id:       String,
    pub holder_address: String,
    pub balance:        BigDecimal,
    pub frozen_balance: BigDecimal,
    pub create_time:    NaiveDateTime,
}

impl SecurityCis2TokenHolder {
    pub fn new(
        cis2_address: &ContractAddress,
        token_id: &cis2::TokenId,
        holder_address: &Address,
        balance: &cis2::TokenAmount,
        frozen_balance: &cis2::TokenAmount,
        create_time: DateTime<Utc>,
    ) -> Self {
        Self {
            cis2_address:   cis2_address.to_string(),
            token_id:       token_id.to_string(),
            holder_address: holder_address.to_string(),
            balance:        token_amount_to_sql(balance),
            frozen_balance: token_amount_to_sql(frozen_balance),
            create_time:    create_time.naive_utc(),
        }
    }
}

pub fn list_tokens_by_holder(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    holder_address: &Address,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<SecurityCis2TokenHolder>, i64)> {
    let query = security_cis2_contract_token_holders::table.filter(
        security_cis2_contract_token_holders::cis2_address.eq(cis2_address.to_string()).and(
            security_cis2_contract_token_holders::holder_address.eq(holder_address.to_string()),
        ),
    );
    let tokens = query
        .clone()
        .select(SecurityCis2TokenHolder::as_select())
        .order(security_cis2_contract_token_holders::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

pub fn list_holders_by_token(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: &cis2::TokenId,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<SecurityCis2TokenHolder>, i64)> {
    let query = security_cis2_contract_token_holders::table.filter(
        security_cis2_contract_token_holders::cis2_address
            .eq(cis2_address.to_string())
            .and(security_cis2_contract_token_holders::token_id.eq(token_id.to_string())),
    );
    let tokens = query
        .clone()
        .select(SecurityCis2TokenHolder::as_select())
        .order(security_cis2_contract_token_holders::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

pub fn update_balance_frozen(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: &cis2::TokenId,
    holder_address: &Address,
    balance_delta: &cis2::TokenAmount,
    increase: bool,
) -> DbResult<()> {
    let update_filter = security_cis2_contract_token_holders::cis2_address
        .eq(cis2_address.to_string())
        .and(security_cis2_contract_token_holders::token_id.eq(token_id.to_string()))
        .and(security_cis2_contract_token_holders::holder_address.eq(holder_address.to_string()));

    let updated_rows = match increase {
        true => {
            let update = security_cis2_contract_token_holders::frozen_balance
                .eq(security_cis2_contract_token_holders::frozen_balance
                    .add(token_amount_to_sql(balance_delta)));
            diesel::update(security_cis2_contract_token_holders::table)
                .filter(update_filter)
                .set(update)
                .execute(conn)?
        }
        false => {
            let update = security_cis2_contract_token_holders::frozen_balance
                .eq(security_cis2_contract_token_holders::frozen_balance
                    .sub(token_amount_to_sql(balance_delta)));
            diesel::update(security_cis2_contract_token_holders::table)
                .filter(update_filter)
                .set(update)
                .execute(conn)?
        }
    };
    assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

    Ok(())
}

pub fn insert_holder_or_add_balance(
    conn: &mut DbConn,
    holder: &SecurityCis2TokenHolder,
) -> DbResult<()> {
    let updated_rows = diesel::insert_into(security_cis2_contract_token_holders::table)
        .values(holder)
        .on_conflict((
            security_cis2_contract_token_holders::cis2_address,
            security_cis2_contract_token_holders::token_id,
            security_cis2_contract_token_holders::holder_address,
        ))
        .do_update()
        .set(
            security_cis2_contract_token_holders::balance
                .eq(security_cis2_contract_token_holders::balance.add(holder.balance.clone())),
        )
        .execute(conn)?;
    assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

    Ok(())
}

pub fn update_sub_balance(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: &cis2::TokenId,
    holder_address: &Address,
    balance_delta: &cis2::TokenAmount,
) -> DbResult<()> {
    conn.transaction(|conn| {
        let update_filter = security_cis2_contract_token_holders::cis2_address
            .eq(cis2_address.to_string())
            .and(security_cis2_contract_token_holders::token_id.eq(token_id.to_string()))
            .and(
                security_cis2_contract_token_holders::holder_address.eq(holder_address.to_string()),
            );
        let update = security_cis2_contract_token_holders::balance
            .eq(security_cis2_contract_token_holders::balance
                .sub(token_amount_to_sql(balance_delta)));
        let updated_rows = diesel::update(security_cis2_contract_token_holders::table)
            .filter(&update_filter)
            .set(update)
            .execute(conn)?;
        assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

        let delete_filter =
            update_filter.and(security_cis2_contract_token_holders::balance.eq(BigDecimal::zero()));
        diesel::delete(security_cis2_contract_token_holders::table)
            .filter(&delete_filter)
            .execute(conn)?;

        Ok(())
    })
}

pub fn update_replace_holder(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    holder_address: &Address,
    recovery_address: &Address,
) -> DbResult<usize> {
    let updated_rows = diesel::update(security_cis2_contract_token_holders::table)
        .filter(
            security_cis2_contract_token_holders::cis2_address.eq(cis2_address.to_string()).and(
                security_cis2_contract_token_holders::holder_address.eq(holder_address.to_string()),
            ),
        )
        .set(security_cis2_contract_token_holders::holder_address.eq(recovery_address.to_string()))
        .execute(conn)?;

    Ok(updated_rows)
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_tokens)]
#[diesel(primary_key(cis2_address, token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2Token {
    pub cis2_address:  String,
    pub token_id:      String,
    pub is_paused:     bool,
    pub metadata_url:  String,
    pub metadata_hash: Option<Vec<u8>>,
    pub supply:        BigDecimal,
    pub create_time:   NaiveDateTime,
}

impl SecurityCis2Token {
    pub fn new(
        cis2_address: &ContractAddress,
        token_id: &cis2::TokenId,
        is_paused: bool,
        metadata_url: String,
        metadata_hash: Option<[u8; 32]>,
        supply: &cis2::TokenAmount,
        create_time: DateTime<Utc>,
    ) -> Self {
        Self {
            cis2_address: cis2_address.to_string(),
            token_id: token_id.to_string(),
            is_paused,
            metadata_url,
            metadata_hash: metadata_hash.map(|value: [u8; 32]| value.to_vec()),
            supply: token_amount_to_sql(supply),
            create_time: create_time.naive_utc(),
        }
    }
}

pub fn list_tokens_for_contract(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<SecurityCis2Token>, i64)> {
    let query = security_cis2_contract_tokens::table
        .filter(security_cis2_contract_tokens::cis2_address.eq(cis2_address.to_string()));
    let tokens = query
        .clone()
        .select(SecurityCis2Token::as_select())
        .order(security_cis2_contract_tokens::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

pub fn insert_token_or_update_metadata(
    conn: &mut DbConn,
    token: &SecurityCis2Token,
) -> DbResult<()> {
    let row_count = diesel::insert_into(security_cis2_contract_tokens::table)
        .values(token)
        .on_conflict((
            security_cis2_contract_tokens::cis2_address,
            security_cis2_contract_tokens::token_id,
        ))
        .do_update()
        .set((
            security_cis2_contract_tokens::metadata_url.eq(token.metadata_url.clone()),
            security_cis2_contract_tokens::metadata_hash.eq(token.metadata_hash.clone()),
        ))
        .execute(conn)?;

    assert_eq!(row_count, 1, "More than 1 row updated");
    Ok(())
}

pub fn update_supply(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: &cis2::TokenId,
    supply_delta: &cis2::TokenAmount,
    increase: bool,
) -> DbResult<()> {
    let update_filter = security_cis2_contract_tokens::cis2_address
        .eq(cis2_address.to_string())
        .and(security_cis2_contract_tokens::token_id.eq(token_id.to_string()));
    let query = diesel::update(security_cis2_contract_tokens::table).filter(&update_filter);
    let update_rows =
        match increase {
            true => query
                .set(security_cis2_contract_tokens::supply.eq(
                    security_cis2_contract_tokens::supply.add(token_amount_to_sql(supply_delta)),
                ))
                .execute(conn)?,
            false => query
                .set(security_cis2_contract_tokens::supply.eq(
                    security_cis2_contract_tokens::supply.sub(token_amount_to_sql(supply_delta)),
                ))
                .execute(conn)?,
        };
    assert_eq!(update_rows, 1, "error {} rows updated", update_rows);
    Ok(())
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_operators)]
#[diesel(primary_key(cis2_address, holder_address, operator_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2Operator {
    pub cis2_address:     String,
    pub holder_address:   String,
    pub operator_address: String,
}
impl SecurityCis2Operator {
    pub fn new(
        cis2_address: &ContractAddress,
        holder_address: &Address,
        operator_address: &Address,
    ) -> Self {
        Self {
            cis2_address:     cis2_address.to_string(),
            holder_address:   holder_address.to_string(),
            operator_address: operator_address.to_string(),
        }
    }
}

pub fn insert_operator(conn: &mut DbConn, record: &SecurityCis2Operator) -> DbResult<()> {
    diesel::insert_into(security_cis2_contract_operators::table)
        .values(record)
        .on_conflict_do_nothing()
        .execute(conn)?;
    Ok(())
}

pub fn delete_operator(conn: &mut DbConn, record: &SecurityCis2Operator) -> DbResult<()> {
    let delete_filter =
        security_cis2_contract_operators::cis2_address.eq(&record.cis2_address).and(
            security_cis2_contract_operators::holder_address
                .eq(&record.holder_address)
                .and(security_cis2_contract_operators::operator_address.eq(&record.holder_address)),
        );

    diesel::delete(security_cis2_contract_operators::table).filter(delete_filter).execute(conn)?;

    Ok(())
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = security_cis2_contract_recovery_records)]
#[diesel(primary_key(cis2_address, holder_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SecurityCis2RecoveryRecord {
    pub cis2_address:      String,
    pub holder_address:    String,
    pub recovered_address: String,
}
impl SecurityCis2RecoveryRecord {
    pub fn new(
        cis2_address: &ContractAddress,
        holder_address: &Address,
        recovered_address: &Address,
    ) -> Self {
        Self {
            cis2_address:      cis2_address.to_string(),
            holder_address:    holder_address.to_string(),
            recovered_address: recovered_address.to_string(),
        }
    }
}

pub fn insert_recovery_record(
    conn: &mut DbConn,
    record: &SecurityCis2RecoveryRecord,
) -> DbResult<()> {
    diesel::insert_into(security_cis2_contract_recovery_records::table)
        .values(record)
        .execute(conn)?;

    Ok(())
}
