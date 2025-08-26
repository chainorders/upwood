use std::ops::{Add, Sub};

use chrono::NaiveDateTime;
use concordium_rust_sdk::types::Address;
use diesel::prelude::*;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    cis2_agents, cis2_compliances, cis2_identity_registries, cis2_operators, cis2_recovery_records,
    cis2_token_holder_balance_updates, cis2_token_holders, cis2_tokens,
};

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    Debug,
    PartialEq,
    Object,
    Serialize,
    AsChangeset,
)]
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
        _time: NaiveDateTime,
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
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let agent = diesel::insert_into(cis2_agents::table)
            .values(self)
            .returning(Agent::as_returning())
            .get_result(conn)?;
        Ok(agent)
    }

    #[instrument(
        skip_all,
        fields(contract = self.cis2_address.to_string(), agent_address = self.agent_address.to_string())
    )]
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let agent = diesel::update(cis2_agents::table)
            .filter(
                cis2_agents::cis2_address
                    .eq(self.cis2_address)
                    .and(cis2_agents::agent_address.eq(self.agent_address.to_string())),
            )
            .set(self)
            .returning(Agent::as_returning())
            .get_result(conn)?;
        Ok(agent)
    }

    #[instrument(
        skip_all,
        fields(contract = self.cis2_address.to_string(), agent_address = self.agent_address.to_string())
    )]
    pub fn delete(&self, conn: &mut DbConn) -> DbResult<()> {
        let delete_filter = cis2_agents::cis2_address
            .eq(&self.cis2_address)
            .and(cis2_agents::agent_address.eq(&self.agent_address));

        diesel::delete(cis2_agents::table)
            .filter(delete_filter)
            .execute(conn)?;

        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Agent>, i64)> {
        let select_filter = cis2_agents::cis2_address.eq(cis2_address);
        let res: Vec<Agent> = cis2_agents::table
            .filter(&select_filter)
            .select(Agent::as_select())
            .limit(page_size)
            .offset(page_size * page)
            .get_results(conn)?;
        let total_count: i64 = cis2_agents::table
            .filter(select_filter)
            .count()
            .get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((res, page_count))
    }

    #[instrument(skip(conn))]
    pub fn find(
        conn: &mut DbConn,
        cis2_address: Decimal,
        agent_address: &str,
    ) -> DbResult<Option<Agent>> {
        let select_filter = cis2_agents::cis2_address
            .eq(cis2_address)
            .and(cis2_agents::agent_address.eq(agent_address));
        let res = cis2_agents::table
            .filter(select_filter)
            .select(Agent::as_select())
            .first::<Agent>(conn)
            .optional()?;
        Ok(res)
    }

    #[instrument(skip(conn))]
    pub fn count(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<i64> {
        let count = cis2_agents::table
            .filter(cis2_agents::cis2_address.eq(cis2_address))
            .count()
            .get_result(conn)?;
        Ok(count)
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = cis2_compliances)]
#[diesel(primary_key(cis2_address, compliance_address))]
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
        diesel::insert_into(cis2_compliances::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn find(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<Option<Compliance>> {
        let compliance = cis2_compliances::table
            .filter(cis2_compliances::cis2_address.eq(cis2_address))
            .first::<Compliance>(conn)
            .optional()?;
        Ok(compliance)
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
        diesel::insert_into(cis2_identity_registries::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn find(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<Option<IdentityRegistry>> {
        let identity_registry = cis2_identity_registries::table
            .filter(cis2_identity_registries::cis2_address.eq(cis2_address))
            .first::<IdentityRegistry>(conn)
            .optional()?;
        Ok(identity_registry)
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Debug,
    PartialEq,
    Object,
    Clone,
    Serialize,
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
    pub update_time:       NaiveDateTime,
}

impl TokenHolder {
    pub fn upsert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let holder = diesel::insert_into(cis2_token_holders::table)
            .values(self)
            .on_conflict((
                cis2_token_holders::cis2_address,
                cis2_token_holders::token_id,
                cis2_token_holders::holder_address,
            ))
            .do_update()
            .set(self)
            .returning(TokenHolder::as_returning())
            .get_result(conn)?;
        Ok(holder)
    }

    pub fn find(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &str,
    ) -> DbResult<Option<TokenHolder>> {
        let holder = cis2_token_holders::table
            .filter(
                cis2_token_holders::cis2_address
                    .eq(cis2_address)
                    .and(cis2_token_holders::token_id.eq(token_id))
                    .and(cis2_token_holders::holder_address.eq(holder_address)),
            )
            .first::<TokenHolder>(conn)
            .optional()?;
        Ok(holder)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let holder = diesel::update(cis2_token_holders::table)
            .filter(
                cis2_token_holders::cis2_address
                    .eq(self.cis2_address)
                    .and(cis2_token_holders::token_id.eq(self.token_id))
                    .and(cis2_token_holders::holder_address.eq(&self.holder_address)),
            )
            .set(self)
            .returning(TokenHolder::as_returning())
            .get_result(conn)?;

        Ok(holder)
    }

    #[instrument(skip_all, fields(token_id = token_id.to_string()))]
    pub fn update_balance_frozen(
        conn: &mut DbConn,
        cis2_address: Decimal,
        token_id: Decimal,
        holder_address: &Address,
        balance_delta: Decimal,
        increase: bool,
    ) -> DbResult<TokenHolder> {
        let update_filter = cis2_token_holders::cis2_address
            .eq(cis2_address)
            .and(cis2_token_holders::token_id.eq(token_id))
            .and(cis2_token_holders::holder_address.eq(holder_address.to_string()));

        let holder = match increase {
            true => diesel::update(cis2_token_holders::table)
                .filter(update_filter)
                .set((
                    cis2_token_holders::frozen_balance
                        .eq(cis2_token_holders::frozen_balance.add(balance_delta)),
                    cis2_token_holders::un_frozen_balance
                        .eq(cis2_token_holders::un_frozen_balance.sub(balance_delta)),
                ))
                .returning(TokenHolder::as_returning())
                .get_result(conn)?,
            false => diesel::update(cis2_token_holders::table)
                .filter(update_filter)
                .set((
                    cis2_token_holders::frozen_balance
                        .eq(cis2_token_holders::frozen_balance.sub(balance_delta)),
                    cis2_token_holders::un_frozen_balance
                        .eq(cis2_token_holders::un_frozen_balance.add(balance_delta)),
                ))
                .returning(TokenHolder::as_returning())
                .get_result(conn)?,
        };

        Ok(holder)
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
    pub fn list(
        conn: &mut DbConn,
        contract: Decimal,
        token_id: Decimal,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<TokenHolder>, i64)> {
        let query = cis2_token_holders::table.filter(
            cis2_token_holders::cis2_address
                .eq(contract)
                .and(cis2_token_holders::token_id.eq(token_id)),
        );
        let holders = query
            .select(TokenHolder::as_select())
            .order(cis2_token_holders::create_time)
            .offset(page * page_size)
            .limit(page_size)
            .get_results(conn)?;
        let total_count: i64 = query.count().get_result(conn)?;
        Ok((holders, total_count))
    }
}

#[derive(
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Debug,
    PartialEq,
    Object,
    Clone,
    Serialize,
)]
#[diesel(table_name = cis2_token_holder_balance_updates)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenHolderBalanceUpdate {
    pub id:                uuid::Uuid,
    #[diesel(deserialize_as = i64)]
    pub id_serial:         Option<i64>,
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
}

impl TokenHolderBalanceUpdate {
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<String>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::cis2_token_holder_balance_updates;
        let query = cis2_token_holder_balance_updates::table;
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        if let Some(cis_address) = cis2_address {
            query = query.filter(cis2_token_holder_balance_updates::cis2_address.eq(cis_address));
            count_query =
                count_query.filter(cis2_token_holder_balance_updates::cis2_address.eq(cis_address));
        }

        if let Some(token) = token_id {
            query = query.filter(cis2_token_holder_balance_updates::token_id.eq(token));
            count_query = count_query.filter(cis2_token_holder_balance_updates::token_id.eq(token));
        }

        if let Some(holder) = holder_address {
            query =
                query.filter(cis2_token_holder_balance_updates::holder_address.eq(holder.clone()));
            count_query =
                count_query.filter(cis2_token_holder_balance_updates::holder_address.eq(holder));
        }

        let results = query
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(conn)?;
        let total_count: i64 = count_query.count().get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((results, page_count))
    }

    #[instrument(skip_all)]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(cis2_token_holder_balance_updates::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(
    diesel_derive_enum::DbEnum, Debug, PartialEq, Enum, Clone, Copy, Serialize, Deserialize, Eq,
)]
#[ExistingTypePath = "crate::schema::sql_types::Cis2TokenHolderBalanceUpdateType"]
pub enum TokenHolderBalanceUpdateType {
    Mint,
    Burn,
    TransferOut,
    TransferIn,
    Freeze,
    UnFreeze,
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
    let total_count: i64 = query.count().get_result(conn)?;
    let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
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
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
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
    pub fn update(&self, conn: &mut DbConn) -> DbResult<Self> {
        let token = diesel::update(cis2_tokens::table)
            .filter(
                cis2_tokens::cis2_address
                    .eq(self.cis2_address)
                    .and(cis2_tokens::token_id.eq(self.token_id)),
            )
            .set(self)
            .returning(Token::as_returning())
            .get_result(conn)?;
        Ok(token)
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

    #[instrument(skip_all)]
    pub fn total_burned(
        conn: &mut DbConn,
        holder_address: &str,
        cis2_address: Decimal,
        token_id: Decimal,
        now: NaiveDateTime,
    ) -> DbResult<Decimal> {
        let burned: Decimal = cis2_token_holder_balance_updates::table
            .filter(
                cis2_token_holder_balance_updates::holder_address
                    .eq(holder_address)
                    .and(cis2_token_holder_balance_updates::cis2_address.eq(cis2_address))
                    .and(cis2_token_holder_balance_updates::token_id.eq(token_id))
                    .and(
                        cis2_token_holder_balance_updates::update_type
                            .eq(TokenHolderBalanceUpdateType::Burn),
                    )
                    .and(cis2_token_holder_balance_updates::create_time.le(now)),
            )
            .select(diesel::dsl::sum(cis2_token_holder_balance_updates::amount))
            .first::<Option<Decimal>>(conn)?
            .unwrap_or(Decimal::ZERO);
        Ok(burned)
    }

    #[instrument(skip_all)]
    pub fn delete(self, conn: &mut DbConn) -> DbResult<()> {
        let deleted_rows = diesel::delete(cis2_tokens::table)
            .filter(
                cis2_tokens::cis2_address
                    .eq(self.cis2_address)
                    .and(cis2_tokens::token_id.eq(self.token_id)),
            )
            .execute(conn)?;
        assert_eq!(deleted_rows, 1, "error {} rows were deleted", deleted_rows);
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Option<Decimal>,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Token>, i64)> {
        let query = cis2_tokens::table;
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();
        if let Some(cis2_address) = cis2_address {
            query = query.filter(cis2_tokens::cis2_address.eq(cis2_address));
            count_query = count_query.filter(cis2_tokens::cis2_address.eq(cis2_address));
        }
        let query = query.order(cis2_tokens::create_time.desc());
        let total_count: i64 = count_query.count().get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        let tokens = query
            .select(Token::as_select())
            .offset(page * page_size)
            .limit(page_size)
            .get_results(conn)?;
        Ok((tokens, page_count))
    }

    #[instrument(skip(conn))]
    pub fn count(conn: &mut DbConn, cis2_address: Decimal) -> DbResult<i64> {
        let count = cis2_tokens::table
            .filter(cis2_tokens::cis2_address.eq(cis2_address))
            .count()
            .get_result(conn)?;
        Ok(count)
    }
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
