use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::types::Address;
use diesel::dsl::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use shared::db::{DbConn, DbResult};
use tracing::instrument;

use crate::schema::{
    identity_registry_agents, identity_registry_identities, identity_registry_issuers,
};

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = identity_registry_identities)]
#[diesel(primary_key(identity_registry_address, identity_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    identity_registry_address: Decimal,
    identity_address:          String,
    create_time:               NaiveDateTime,
}

impl Identity {
    pub fn new(
        identity_address: &Address,
        time: DateTime<Utc>,
        identity_registry_address: Decimal,
    ) -> Self {
        Self {
            identity_address: identity_address.to_string(),
            create_time: time.naive_utc(),
            identity_registry_address,
        }
    }

    #[allow(dead_code)]
    #[instrument(skip_all)]
    pub fn list(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        page_size: i64,
        page: i64,
    ) -> DbResult<(Vec<Identity>, i64)> {
        let select_filter =
            identity_registry_identities::identity_registry_address.eq(identity_registry_address);
        let res: Vec<Identity> = identity_registry_identities::table
            .filter(select_filter)
            .select(Identity::as_select())
            .limit(page_size)
            .offset(page_size * page)
            .get_results(conn)?;
        let count: i64 = identity_registry_identities::table
            .filter(select_filter)
            .count()
            .get_result(conn)?;
        let page_count = (count + page_size - 1) / page_size;

        Ok((res, page_count))
    }

    #[instrument(skip_all)]
    pub fn find(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        address: &Address,
    ) -> DbResult<Option<Identity>> {
        let select_filter = identity_registry_identities::identity_registry_address
            .eq(identity_registry_address)
            .and(identity_registry_identities::identity_address.eq(address.to_string()));
        identity_registry_identities::table
            .filter(select_filter)
            .select(Identity::as_select())
            .first(conn)
            .optional()
    }

    #[instrument(skip_all)]
    pub fn exists(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        address: &Address,
    ) -> DbResult<bool> {
        let address = address.to_string();
        let res: bool = select(exists(
            identity_registry_identities::table.filter(
                identity_registry_identities::identity_registry_address
                    .eq(identity_registry_address)
                    .and(identity_registry_identities::identity_address.eq(address.to_string())),
            ),
        ))
        .get_result(conn)?;
        Ok(res)
    }

    #[instrument(skip_all)]
    pub fn exists_batch(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        addresses: &[Address],
    ) -> DbResult<Vec<Address>> {
        let select_filter = identity_registry_identities::identity_registry_address
            .eq(identity_registry_address)
            .and(
                identity_registry_identities::identity_address
                    .eq_any(addresses.iter().map(|a| a.to_string())),
            );
        let res = identity_registry_identities::table
            .filter(select_filter)
            .select(identity_registry_identities::identity_address)
            .load::<String>(conn)?
            .into_iter()
            .map(|a| a.parse().expect("invalid identity registry address"))
            .collect::<Vec<Address>>();
        Ok(res)
    }

    #[instrument(
        skip_all,
        fields(identity_registry = self.identity_registry_address.to_string(), address = self.identity_address.to_string())
    )]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<usize> {
        diesel::insert_into(identity_registry_identities::table)
            .values(self)
            .execute(conn)
    }

    #[instrument(skip_all, fields(identity_address = address.to_string()))]
    pub fn delete(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        address: &Address,
    ) -> DbResult<usize> {
        diesel::delete(QueryDsl::filter(
            identity_registry_identities::table,
            identity_registry_identities::identity_address
                .eq(address.to_string())
                .and(
                    identity_registry_identities::identity_registry_address
                        .eq(identity_registry_address),
                ),
        ))
        .execute(conn)
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = identity_registry_issuers)]
#[diesel(primary_key(identity_registry_address, issuer_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issuer {
    identity_registry_address: Decimal,
    issuer_address:            Decimal,
    create_time:               NaiveDateTime,
}

impl Issuer {
    pub fn new(
        issuer_address: Decimal,
        time: DateTime<Utc>,
        identity_registry_address: Decimal,
    ) -> Self {
        Issuer {
            issuer_address,
            create_time: time.naive_utc(),
            identity_registry_address,
        }
    }

    #[instrument(skip(conn))]
    #[allow(dead_code)]
    pub fn list(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        page_size: i64,
        page: i64,
    ) -> DbResult<(Vec<Issuer>, i64)> {
        let select_filter =
            identity_registry_issuers::identity_registry_address.eq(identity_registry_address);
        let res: Vec<Issuer> = identity_registry_issuers::table
            .filter(select_filter)
            .select(Issuer::as_select())
            .limit(page_size)
            .offset(page_size * page)
            .get_results(conn)?;
        let count: i64 = identity_registry_issuers::table
            .filter(select_filter)
            .count()
            .get_result(conn)?;
        let page_count = (count + page_size - 1) / page_size;

        Ok((res, page_count))
    }

    #[instrument(
        skip_all,
        fields(identity_registry = self.identity_registry_address.to_string(), address = self.issuer_address.to_string()))
    ]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<usize> {
        diesel::insert_into(identity_registry_issuers::table)
            .values(self)
            .execute(conn)
    }

    #[instrument(
        skip_all,
        fields(identity_registry = identity_registry_address.to_string(),address = issuer_address.to_string()))
    ]
    pub fn delete(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        issuer_address: Decimal,
    ) -> DbResult<usize> {
        let delete_filter = identity_registry_issuers::issuer_address
            .eq(issuer_address)
            .and(
                identity_registry_issuers::identity_registry_address.eq(identity_registry_address),
            );

        diesel::delete(identity_registry_issuers::table)
            .filter(delete_filter)
            .execute(conn)
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = identity_registry_agents)]
#[diesel(primary_key(identity_registry_address, agent_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub identity_registry_address: Decimal,
    pub agent_address:             String,
    pub create_time:               NaiveDateTime,
}

impl Agent {
    pub fn new(address: Address, time: DateTime<Utc>, identity_registry_address: Decimal) -> Self {
        Self {
            agent_address: address.to_string(),
            create_time: time.naive_utc(),
            identity_registry_address,
        }
    }

    #[allow(dead_code)]
    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        page_size: i64,
        page: i64,
    ) -> DbResult<(Vec<Agent>, i64)> {
        let select_filter =
            identity_registry_agents::identity_registry_address.eq(identity_registry_address);
        let res: Vec<Agent> = identity_registry_agents::table
            .filter(select_filter)
            .select(Agent::as_select())
            .limit(page_size)
            .offset(page_size * page)
            .get_results(conn)?;
        let count: i64 = identity_registry_agents::table
            .filter(select_filter)
            .count()
            .get_result(conn)?;
        let page_count = (count + page_size - 1) / page_size;

        Ok((res, page_count))
    }

    #[instrument(
        skip_all,
        fields(identity_registry_address=self.identity_registry_address.to_string(), agent_address = self.agent_address)
    )]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<usize> {
        diesel::insert_into(identity_registry_agents::table)
            .values(self)
            .execute(conn)
    }

    #[instrument(skip_all, fields(identity_registry_address, agent_address))]
    pub fn delete(
        conn: &mut DbConn,
        identity_registry_address: Decimal,
        agent_address: &Address,
    ) -> DbResult<usize> {
        let delete_filter = identity_registry_agents::agent_address
            .eq(agent_address.to_string())
            .and(identity_registry_agents::identity_registry_address.eq(identity_registry_address));

        diesel::delete(identity_registry_agents::table)
            .filter(delete_filter)
            .execute(conn)
    }
}
