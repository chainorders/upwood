use std::ops::{Add, Sub};

use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::Address;
use diesel::prelude::*;
use num_traits::{ToPrimitive, Zero};
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::Serialize;
use shared::db::{DbConn, DbResult};
use tracing::instrument;

use crate::schema::{
    cis2_agents, cis2_compliances, cis2_identity_registries, cis2_operators, cis2_recovery_records,
    cis2_token_holders, cis2_tokens,
};

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq, Object, Serialize)]
#[diesel(table_name = cis2_agents)]
#[diesel(primary_key(cis2_address, agent_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub cis2_address:  Decimal,
    pub agent_address: String,
    pub roles:         Vec<Option<String>>,
}

impl Agent {
    pub fn new(
        agent_address: Address,
        _time: DateTime<Utc>,
        cis2_address: Decimal,
        roles: Vec<String>,
    ) -> Self {
        Self {
            agent_address: agent_address.to_string(),
            cis2_address,
            roles: roles.into_iter().map(Some).collect(),
        }
    }

    #[instrument(
        skip_all,
        fields(contract = self.cis2_address.to_string(), agent_address = self.agent_address.to_string())
    )]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        let updated_rows = diesel::insert_into(cis2_agents::table)
            .values(self)
            .execute(conn)?;
        assert_eq!(updated_rows, 1, "error {} rows were updated", updated_rows);
        Ok(())
    }

    #[instrument(
        skip_all,
        fields(contract = cis2_address.to_string(), agent_address = agent_address.to_string())
    )]
    pub fn delete(
        conn: &mut DbConn,
        cis2_address: Decimal,
        agent_address: &Address,
    ) -> DbResult<()> {
        let delete_filter = cis2_agents::cis2_address
            .eq(cis2_address)
            .and(cis2_agents::agent_address.eq(agent_address.to_string()));
        let updated_rows = diesel::delete(cis2_agents::table)
            .filter(delete_filter)
            .execute(conn)?;
        assert_eq!(updated_rows, 1, "error {} rows were updated", updated_rows);
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Decimal,
        page_size: i64,
        page: i64,
    ) -> DbResult<(Vec<Agent>, i64)> {
        let select_filter = cis2_agents::cis2_address.eq(cis2_address);
        let res: Vec<Agent> = cis2_agents::table
            .filter(&select_filter)
            .select(Agent::as_select())
            .limit(page_size)
            .offset(page_size * page)
            .get_results(conn)?;
        let count: i64 = cis2_agents::table
            .filter(select_filter)
            .count()
            .get_result(conn)?;
        let page_count = (count + page_size - 1) / page_size;

        Ok((res, page_count))
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = cis2_compliances)]
#[diesel(primary_key(cis2_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Compliance {
    pub cis2_address:       Decimal,
    pub compliance_address: String,
}

impl Compliance {
    pub fn new(cis2_address: Decimal, compliance: Decimal) -> Self {
        Compliance {
            cis2_address,
            compliance_address: compliance.to_string(),
        }
    }

    #[instrument(
        skip_all,
        fields(compliance_address = self.compliance_address.to_string())
    )]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        let row_count = diesel::insert_into(cis2_compliances::table)
            .values(self)
            .on_conflict((cis2_compliances::cis2_address,))
            .do_update()
            .set(self)
            .execute(conn)?;

        assert_eq!(row_count, 1, "More than one row updated");

        Ok(())
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name =
cis2_identity_registries)]
#[diesel(primary_key(cis2_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IdentityRegistry {
    pub cis2_address:              Decimal,
    pub identity_registry_address: Decimal,
}

impl IdentityRegistry {
    pub fn new(cis2_address: Decimal, identity_registry_address: Decimal) -> Self {
        IdentityRegistry {
            cis2_address,
            identity_registry_address,
        }
    }

    #[instrument(
        skip_all,
        fields(identity_registry_address = self.identity_registry_address.to_string())
    )]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        let row_count = diesel::insert_into(cis2_identity_registries::table)
            .values(self)
            .on_conflict((cis2_identity_registries::cis2_address,))
            .do_update()
            .set(self)
            .execute(conn)?;

        assert_eq!(row_count, 1, "More than one row updated");

        Ok(())
    }
}

#[derive(
    Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Object, Clone,
)]
#[diesel(table_name = cis2_token_holders)]
#[diesel(primary_key(cis2_address, token_id, holder_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenHolder {
    pub cis2_address:      Decimal,
    pub token_id:          Decimal,
    pub holder_address:    String,
    pub frozen_balance:    Decimal,
    pub un_frozen_balance: Decimal,
    pub create_time:       NaiveDateTime,
}

impl TokenHolder {
    pub fn new(
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &Address,
        balance: Decimal,
        create_time: DateTime<Utc>,
    ) -> Self {
        Self {
            token_id,
            cis2_address,
            holder_address: holder_address.to_string(),
            un_frozen_balance: balance,
            frozen_balance: Decimal::ZERO,
            create_time: create_time.naive_utc(),
        }
    }

    #[instrument(skip_all, fields(holder = self.holder_address.to_string()))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        let updated_rows = diesel::insert_into(cis2_token_holders::table)
            .values(self)
            .on_conflict((
                cis2_token_holders::cis2_address,
                cis2_token_holders::token_id,
                cis2_token_holders::holder_address,
            ))
            .do_update()
            .set(
                cis2_token_holders::un_frozen_balance
                    .eq(cis2_token_holders::un_frozen_balance.add(&self.un_frozen_balance)),
            )
            .execute(conn)?;
        assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

        Ok(())
    }

    #[instrument(skip_all, fields(holder = holder_address.to_string()))]
    pub fn sub_balance_unfrozen(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &Address,
        balance_delta: Decimal,
    ) -> DbResult<()> {
        conn.transaction(|conn| {
            let update_filter = cis2_token_holders::cis2_address
                .eq(cis2_address)
                .and(cis2_token_holders::token_id.eq(token_id))
                .and(cis2_token_holders::holder_address.eq(holder_address.to_string()));
            let update = cis2_token_holders::un_frozen_balance
                .eq(cis2_token_holders::un_frozen_balance.sub(balance_delta));
            let updated_rows = diesel::update(cis2_token_holders::table)
                .filter(&update_filter)
                .set(update)
                .execute(conn)?;
            assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

            let delete_filter = update_filter
                .and(cis2_token_holders::un_frozen_balance.eq(Decimal::zero()))
                .and(cis2_token_holders::frozen_balance.eq(Decimal::zero()));
            diesel::delete(cis2_token_holders::table)
                .filter(&delete_filter)
                .execute(conn)?;

            Ok(())
        })
    }

    #[instrument(skip_all, fields(holder = holder_address.to_string()))]
    pub fn balance_of(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &Address,
    ) -> DbResult<u64> {
        let balance = cis2_token_holders::table
            .filter(
                cis2_token_holders::cis2_address
                    .eq(cis2_address)
                    .and(cis2_token_holders::token_id.eq(token_id))
                    .and(cis2_token_holders::holder_address.eq(holder_address.to_string())),
            )
            .select(cis2_token_holders::un_frozen_balance.add(cis2_token_holders::frozen_balance))
            .first::<Decimal>(conn)?;
        let balance = balance
            .to_u64()
            .unwrap_or_else(|| panic!("{} is not a valid u64", balance.to_string().as_str()));
        Ok(balance)
    }

    #[instrument(skip_all, fields(token_id = token_id.to_string()))]
    pub fn update_balance_frozen(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &Address,
        balance_delta: Decimal,
        increase: bool,
    ) -> DbResult<()> {
        let update_filter = cis2_token_holders::cis2_address
            .eq(cis2_address)
            .and(cis2_token_holders::token_id.eq(token_id))
            .and(cis2_token_holders::holder_address.eq(holder_address.to_string()));

        let updated_rows = match increase {
            true => diesel::update(cis2_token_holders::table)
                .filter(update_filter)
                .set((
                    cis2_token_holders::frozen_balance
                        .eq(cis2_token_holders::frozen_balance.add(balance_delta)),
                    cis2_token_holders::frozen_balance
                        .eq(cis2_token_holders::un_frozen_balance.sub(balance_delta)),
                ))
                .execute(conn)?,
            false => diesel::update(cis2_token_holders::table)
                .filter(update_filter)
                .set((
                    cis2_token_holders::frozen_balance
                        .eq(cis2_token_holders::frozen_balance.sub(balance_delta)),
                    cis2_token_holders::un_frozen_balance
                        .eq(cis2_token_holders::un_frozen_balance.add(balance_delta)),
                ))
                .execute(conn)?,
        };
        assert_eq!(updated_rows, 1, "error: {} rows(s) updated", updated_rows);

        Ok(())
    }

    #[instrument(skip_all, fields(holder = holder_address.to_string()))]
    pub fn replace(
        conn: &mut DbConn,
        cis2_address: Decimal,
        holder_address: &Address,
        recovery_address: &Address,
    ) -> DbResult<usize> {
        let updated_rows = diesel::update(cis2_token_holders::table)
            .filter(
                cis2_token_holders::cis2_address
                    .eq(cis2_address)
                    .and(cis2_token_holders::holder_address.eq(holder_address.to_string())),
            )
            .set(cis2_token_holders::holder_address.eq(recovery_address.to_string()))
            .execute(conn)?;

        Ok(updated_rows)
    }

    #[instrument(skip(conn))]
    pub fn list_by_tokens(
        conn: &mut DbConn,
        tokens: Vec<(Decimal, Decimal)>,
        holder_address: &Address,
    ) -> DbResult<Vec<TokenHolder>> {
        let contract_addresses = tokens.iter().map(|(a, _)| a);
        let token_ids = tokens.iter().map(|(_, t)| t);
        let holders = cis2_token_holders::table
            .filter(
                cis2_token_holders::cis2_address
                    .eq_any(contract_addresses)
                    .and(cis2_token_holders::token_id.eq_any(token_ids))
                    .and(cis2_token_holders::holder_address.eq(holder_address.to_string()))
                    .and(cis2_token_holders::un_frozen_balance.gt(Decimal::zero())),
            )
            .select(TokenHolder::as_select())
            .get_results(conn)?;
        Ok(holders)
    }
}

#[instrument(skip(conn))]
pub fn list_tokens_by_holder(
    conn: &mut DbConn,
    cis2_address: Decimal,
    holder_address: &Address,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<TokenHolder>, i64)> {
    let query = cis2_token_holders::table.filter(
        cis2_token_holders::cis2_address
            .eq(cis2_address)
            .and(cis2_token_holders::holder_address.eq(holder_address.to_string())),
    );
    let tokens = query
        .clone()
        .select(TokenHolder::as_select())
        .order(cis2_token_holders::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

#[instrument(skip_all)]
pub fn list_holders_by_token_metadata_url(
    conn: &mut DbConn,
    cis2_address: Decimal,
    metadata_url: &str,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<TokenHolder>, i64)> {
    let query = cis2_token_holders::table
        .inner_join(
            cis2_tokens::table.on(cis2_token_holders::token_id
                .eq(cis2_tokens::token_id)
                .and(cis2_tokens::cis2_address.eq(cis2_address))),
        )
        .filter(
            cis2_tokens::cis2_address
                .eq(cis2_address)
                .and(cis2_tokens::metadata_url.eq(metadata_url)),
        );

    let tokens = query
        .select(TokenHolder::as_select())
        .order(cis2_token_holders::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;
    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

#[instrument(skip(conn))]
pub fn list_holders_by_token(
    conn: &mut DbConn,
    cis2_address: Decimal,
    token_id: Decimal,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<TokenHolder>, i64)> {
    let query = cis2_token_holders::table.filter(
        cis2_token_holders::cis2_address
            .eq(cis2_address)
            .and(cis2_token_holders::token_id.eq(token_id)),
    );
    let tokens = query
        .select(TokenHolder::as_select())
        .order(cis2_token_holders::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

#[instrument(skip(conn))]
pub fn holders_count_by_token(
    conn: &mut DbConn,
    cis2_address: Decimal,
    token_id: Decimal,
) -> DbResult<i64> {
    let count: i64 = cis2_token_holders::table
        .filter(
            cis2_token_holders::cis2_address
                .eq(cis2_address)
                .and(cis2_token_holders::token_id.eq(token_id)),
        )
        .count()
        .get_result(conn)?;
    Ok(count)
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq,
)]
#[diesel(table_name = cis2_tokens)]
#[diesel(primary_key(cis2_address, token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub cis2_address:  Decimal,
    pub token_id:      Decimal,
    pub is_paused:     bool,
    pub metadata_url:  String,
    pub metadata_hash: Option<String>,
    pub supply:        Decimal,
    pub create_time:   NaiveDateTime,
    pub update_time:   NaiveDateTime,
}

impl Token {
    pub fn new(
        cis2_address: Decimal,
        token_id: Decimal,
        is_paused: bool,
        metadata_url: String,
        metadata_hash: Option<[u8; 32]>,
        supply: Decimal,
        block_slot_time: DateTime<Utc>,
    ) -> Self {
        Self {
            cis2_address,
            token_id,
            is_paused,
            metadata_url,
            metadata_hash: metadata_hash.map(hex::encode),
            supply,
            create_time: block_slot_time.naive_utc(),
            update_time: block_slot_time.naive_utc(),
        }
    }

    #[instrument(skip_all, fields(token_id = self.token_id.to_string()))]
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(cis2_tokens::table)
            .values(self)
            .on_conflict((cis2_tokens::cis2_address, cis2_tokens::token_id))
            .do_update()
            .set((
                cis2_tokens::metadata_url.eq(&self.metadata_url),
                cis2_tokens::metadata_hash.eq(&self.metadata_hash),
                cis2_tokens::update_time.eq(&self.update_time),
            ))
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all, fields(token_id))]
    pub fn update_supply(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        supply_delta: Decimal,
        increase: bool,
    ) -> DbResult<()> {
        let update_filter = cis2_tokens::cis2_address
            .eq(cis2_address)
            .and(cis2_tokens::token_id.eq(token_id));
        let query = diesel::update(cis2_tokens::table).filter(&update_filter);
        let update_rows = match increase {
            true => query
                .set(cis2_tokens::supply.eq(cis2_tokens::supply.add(supply_delta)))
                .execute(conn)?,
            false => query
                .set(cis2_tokens::supply.eq(cis2_tokens::supply.sub(supply_delta)))
                .execute(conn)?,
        };
        assert_eq!(update_rows, 1, "error {} rows updated", update_rows);
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn find(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
    ) -> DbResult<Option<Token>> {
        let token = cis2_tokens::table
            .filter(cis2_tokens::cis2_address.eq(cis2_address))
            .filter(cis2_tokens::token_id.eq(token_id))
            .first::<Token>(conn)
            .optional()?;
        Ok(token)
    }

    #[instrument(skip_all, fields(token_id = token_id.to_string()))]
    pub fn update_paused(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        paused: bool,
    ) -> DbResult<()> {
        let update_filter = cis2_tokens::cis2_address
            .eq(cis2_address)
            .and(cis2_tokens::token_id.eq(token_id));
        let update = cis2_tokens::is_paused.eq(paused);

        let row_count = diesel::update(cis2_tokens::table)
            .filter(update_filter)
            .set(update)
            .execute(conn)?;
        assert_eq!(row_count, 1, "More than one row updated");

        Ok(())
    }
}

#[instrument(skip(conn))]
pub fn list_tokens_for_contract(
    conn: &mut DbConn,
    cis2_address: Decimal,
    page_size: i64,
    page: i64,
) -> DbResult<(Vec<Token>, i64)> {
    let query = cis2_tokens::table.filter(cis2_tokens::cis2_address.eq(cis2_address));
    let tokens = query
        .select(Token::as_select())
        .order(cis2_tokens::create_time)
        .offset(page * page_size)
        .limit(page_size)
        .get_results(conn)?;
    let count_total: i64 = query.count().get_result(conn)?;

    let page_count = (count_total + page_size - 1) / page_size;
    Ok((tokens, page_count))
}

#[instrument(skip_all)]
pub fn tokens_count(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<u64> {
    let count: i64 = cis2_tokens::table
        .filter(cis2_tokens::cis2_address.eq(cis2_address))
        .count()
        .get_result(conn)?;
    Ok(count as u64)
}

#[instrument(skip_all)]
pub fn tokens_by_holder_for_contracts(
    conn: &mut DbConn,
    owner: &AccountAddress,
    ci2_addresses: &[Decimal],
) -> DbResult<Vec<TokenHolder>> {
    let tokens = cis2_tokens::table
        .filter(cis2_tokens::cis2_address.eq_any(ci2_addresses))
        .inner_join(
            cis2_token_holders::table.on(cis2_tokens::cis2_address
                .eq(cis2_token_holders::cis2_address)
                .and(cis2_tokens::token_id.eq(cis2_token_holders::token_id))),
        )
        .filter(cis2_token_holders::holder_address.eq(owner.to_string()))
        .select(TokenHolder::as_select())
        .get_results(conn)?;
    Ok(tokens)
}

#[instrument(skip_all)]
pub fn metadata_count(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<u64> {
    let count: i64 = cis2_tokens::table
        .filter(cis2_tokens::cis2_address.eq(cis2_address))
        .filter(cis2_tokens::metadata_url.is_not_null())
        .select(cis2_tokens::metadata_url)
        .count()
        .distinct()
        .get_result(conn)?;
    Ok(count as u64)
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = cis2_operators)]
#[diesel(primary_key(cis2_address, holder_address, operator_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Operator {
    pub cis2_address:     Decimal,
    pub holder_address:   String,
    pub operator_address: String,
}
impl Operator {
    pub fn new(
        cis2_address: Decimal,
        holder_address: &Address,
        operator_address: &Address,
    ) -> Self {
        Self {
            cis2_address,
            holder_address: holder_address.to_string(),
            operator_address: operator_address.to_string(),
        }
    }

    #[instrument(skip_all, fields(holder_address))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(cis2_operators::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip_all, fields(holder_address))]
    pub fn delete(&self, conn: &mut DbConn) -> DbResult<()> {
        let delete_filter = cis2_operators::cis2_address.eq(&self.cis2_address).and(
            cis2_operators::holder_address
                .eq(&self.holder_address)
                .and(cis2_operators::operator_address.eq(&self.holder_address)),
        );

        diesel::delete(cis2_operators::table)
            .filter(delete_filter)
            .execute(conn)?;

        Ok(())
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = cis2_recovery_records)]
#[diesel(primary_key(cis2_address, holder_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecoveryRecord {
    pub cis2_address:      Decimal,
    pub holder_address:    String,
    pub recovered_address: String,
}
impl RecoveryRecord {
    pub fn new(
        cis2_address: Decimal,
        holder_address: &Address,
        recovered_address: &Address,
    ) -> Self {
        Self {
            cis2_address,
            holder_address: holder_address.to_string(),
            recovered_address: recovered_address.to_string(),
        }
    }

    #[instrument(skip_all, fields(self))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(cis2_recovery_records::table)
            .values(self)
            .execute(conn)?;

        Ok(())
    }
}
