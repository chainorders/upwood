use crate::{
    schema::{identity_registry_agents, identity_registry_identities, identity_registry_issuers},
    shared::db::{address_to_sql_string, DbConn, DbResult},
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::types::{Address, ContractAddress};
use diesel::prelude::*;

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = identity_registry_identities)]
#[diesel(primary_key(contract_index, contract_sub_index, identity_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    identity_address:   String,
    create_time:        NaiveDateTime,
    contract_index:     BigDecimal,
    contract_sub_index: BigDecimal,
}

impl Identity {
    pub fn new(address: Address, time: DateTime<Utc>, contract: &ContractAddress) -> Self {
        Self {
            identity_address:   address_to_sql_string(&address),
            create_time:        time.naive_utc(),
            contract_index:     contract.index.into(),
            contract_sub_index: contract.subindex.into(),
        }
    }
}

#[allow(dead_code)]
pub fn list_identities(
    conn: &mut DbConn,
    identity_registry_contract: &ContractAddress,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<Identity>, i64)> {
    let select_filter = identity_registry_identities::contract_index
        .eq::<BigDecimal>(identity_registry_contract.index.into())
        .and(
            identity_registry_identities::contract_sub_index
                .eq::<BigDecimal>(identity_registry_contract.subindex.into()),
        );
    let res: Vec<Identity> = identity_registry_identities::table
        .filter(select_filter.clone())
        .select(Identity::as_select())
        .limit(page_size)
        .offset(page_size * page)
        .get_results(conn)?;
    let count: i64 =
        identity_registry_identities::table.filter(select_filter).count().get_result(conn)?;
    let page_count = (count + page_size - 1) / page_size;

    Ok((res, page_count))
}

pub fn insert_identity(conn: &mut DbConn, identity: Identity) -> DbResult<usize> {
    diesel::insert_into(identity_registry_identities::table).values(identity).execute(conn)
}

pub fn remove_identity(conn: &mut DbConn, address: &Address) -> DbResult<usize> {
    diesel::delete(QueryDsl::filter(
        identity_registry_identities::table,
        identity_registry_identities::identity_address.eq(address_to_sql_string(address)),
    ))
    .execute(conn)
}

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(table_name = identity_registry_issuers)]
#[diesel(primary_key(contract_index, contract_sub_index, issuer_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issuer {
    issuer_address:     String,
    create_time:        NaiveDateTime,
    contract_index:     BigDecimal,
    contract_sub_index: BigDecimal,
}

impl Issuer {
    pub fn new(address: ContractAddress, time: DateTime<Utc>, contract: &ContractAddress) -> Self {
        Issuer {
            issuer_address:     address_to_sql_string(&address.into()),
            create_time:        time.naive_utc(),
            contract_index:     contract.index.into(),
            contract_sub_index: contract.subindex.into(),
        }
    }
}

pub fn insert_issuer(conn: &mut DbConn, agent: Issuer) -> DbResult<usize> {
    diesel::insert_into(identity_registry_issuers::table).values(agent).execute(conn)
}

pub fn remove_issuer(conn: &mut DbConn, address: ContractAddress) -> DbResult<usize> {
    diesel::delete(QueryDsl::filter(
        identity_registry_issuers::table,
        identity_registry_issuers::issuer_address
            .eq(address_to_sql_string(&Address::Contract(address))),
    ))
    .execute(conn)
}

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(table_name = identity_registry_agents)]
#[diesel(primary_key(contract_index, contract_sub_index, agent_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub agent_address:      String,
    pub create_time:        NaiveDateTime,
    pub contract_index:     BigDecimal,
    pub contract_sub_index: BigDecimal,
}

impl Agent {
    pub fn new(address: Address, time: DateTime<Utc>, contract: &ContractAddress) -> Self {
        Self {
            agent_address:      address_to_sql_string(&address),
            create_time:        time.naive_utc(),
            contract_index:     contract.index.into(),
            contract_sub_index: contract.subindex.into(),
        }
    }
}

pub fn insert_agent(conn: &mut DbConn, agent: Agent) -> DbResult<usize> {
    diesel::insert_into(identity_registry_agents::table).values(agent).execute(conn)
}

pub fn remove_agent(conn: &mut DbConn, address: &Address) -> DbResult<usize> {
    diesel::delete(QueryDsl::filter(
        identity_registry_agents::table,
        identity_registry_agents::agent_address.eq(address_to_sql_string(address)),
    ))
    .execute(conn)
}
