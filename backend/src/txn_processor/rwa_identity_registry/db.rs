use crate::{
    schema::{identity_registry_agents, identity_registry_identities, identity_registry_issuers},
    shared::db::{address_to_sql_string, DbConn, DbResult},
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::types::{Address, ContractAddress};
use diesel::{dsl::*, prelude::*};

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(table_name = identity_registry_identities)]
#[diesel(primary_key(identity_address))]
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

pub fn insert_identity(conn: &mut DbConn, identity: Identity) -> DbResult<usize> {
    insert_into(identity_registry_identities::table).values(identity).execute(conn)
}

pub fn remove_identity(conn: &mut DbConn, address: &Address) -> DbResult<usize> {
    delete(QueryDsl::filter(
        identity_registry_identities::table,
        identity_registry_identities::identity_address.eq(address_to_sql_string(address)),
    ))
    .execute(conn)
}

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(table_name = identity_registry_issuers)]
#[diesel(primary_key(issuer_address))]
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
    insert_into(identity_registry_issuers::table).values(agent).execute(conn)
}

pub fn remove_issuer(conn: &mut DbConn, address: ContractAddress) -> DbResult<usize> {
    delete(QueryDsl::filter(
        identity_registry_issuers::table,
        identity_registry_issuers::issuer_address
            .eq(address_to_sql_string(&Address::Contract(address))),
    ))
    .execute(conn)
}

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(table_name = identity_registry_agents)]
#[diesel(primary_key(agent_address))]
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
    insert_into(identity_registry_agents::table).values(agent).execute(conn)
}

pub fn remove_agent(conn: &mut DbConn, address: &Address) -> DbResult<usize> {
    delete(QueryDsl::filter(
        identity_registry_agents::table,
        identity_registry_agents::agent_address.eq(address_to_sql_string(address)),
    ))
    .execute(conn)
}
