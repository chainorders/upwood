use std::io::Write;

use chrono::NaiveDateTime;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::ContractAddress;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::dsl::*;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use poem_openapi::{Enum, Object};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::security_mint_fund::{SecurityMintFundContract, SecurityMintFundState};
use crate::db::security_p2p_trading::P2PTradeContract;
use crate::db_shared::{DbConn, DbResult};
use crate::schema;

pub const TRACKED_TOKEN_ID: Decimal = Decimal::ZERO;

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    AsChangeset,
    Deserialize,
    Clone,
)]
#[diesel(table_name = crate::schema::forest_projects)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProject {
    pub id: uuid::Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_large_url: String,
    pub image_small_url: String,
    pub geo_spatial_url: Option<String>,
    pub contract_address: Decimal,
    pub mint_fund_contract_address: Option<Decimal>,
    pub p2p_trade_contract_address: Option<Decimal>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    pub latest_price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ForestProject {
    pub fn mint_fund_contract_address(&self) -> Option<ContractAddress> {
        self.mint_fund_contract_address
            .map(|a| ContractAddress::new(a.to_u64().unwrap(), 0))
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        let project = diesel::insert_into(schema::forest_projects::table)
            .values(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)?;
        Ok(project)
    }

    pub fn update(&self, conn: &mut DbConn) -> DbResult<ForestProject> {
        let project = diesel::update(schema::forest_projects::table)
            .filter(schema::forest_projects::id.eq(self.id))
            .set(self)
            .returning(ForestProject::as_returning())
            .get_result(conn)?;
        Ok(project)
    }

    pub fn find(conn: &mut DbConn, project_id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_projects::table
            .filter(schema::forest_projects::id.eq(project_id))
            .select(ForestProject::as_select())
            .first::<Self>(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        state: Option<ForestProjectState>,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let query = schema::forest_projects::table.into_boxed();
        let count_query = schema::forest_projects::table.into_boxed();
        let (query, count_query) = match state {
            Some(state) => (
                query.filter(schema::forest_projects::state.eq(state)),
                count_query.filter(schema::forest_projects::state.eq(state)),
            ),
            None => (query, count_query),
        };

        let projects = query
            .select(ForestProject::as_select())
            .order(schema::forest_projects::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn mint_fund(&self, conn: &mut DbConn) -> DbResult<Option<SecurityMintFundContract>> {
        match self.mint_fund_contract_address {
            Some(mint_fund_contract_address) => {
                let contract = SecurityMintFundContract::find(conn, mint_fund_contract_address)?;
                Ok(contract)
            }
            None => Ok(None),
        }
    }

    pub fn p2p_trade(&self, conn: &mut DbConn) -> DbResult<Option<P2PTradeContract>> {
        match self.p2p_trade_contract_address {
            Some(p2p_trade_contract_address) => {
                let contract = P2PTradeContract::find(conn, p2p_trade_contract_address)?;
                Ok(contract)
            }
            None => Ok(None),
        }
    }

    pub fn user_notification_exists(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<bool> {
        Notification::exists(conn, self.id, user_cognito_id)
    }

    pub fn user_notification(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<Option<Notification>> {
        Notification::find(conn, self.id, user_cognito_id)
    }

    pub fn legal_contract(&self, conn: &mut DbConn) -> DbResult<Option<LegalContract>> {
        LegalContract::find(conn, self.id)
    }

    pub fn user_legal_contract(
        &self,
        conn: &mut DbConn,
        user_cognito_id: &str,
    ) -> DbResult<Option<LegalContractUserSignature>> {
        LegalContractUserSignature::find(conn, self.id, user_cognito_id)
    }

    pub fn media(
        &self,
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<ForestProjectMedia>, i64)> {
        ForestProjectMedia::list(conn, self.id, page, page_size)
    }
}

diesel::table! {
    use diesel::sql_types::*;

    forest_project_user_view (
        id,
        contract_address,
        notification_cognito_user_id,
        legal_contract_signer,
        project_token_holder_address
    ) {
        id -> Uuid,
        name -> Varchar,
        label -> Varchar,
        desc_short -> Text,
        desc_long -> Text,
        area -> Varchar,
        carbon_credits -> Int4,
        roi_percent -> Float4,
        state -> crate::schema::sql_types::ForestProjectState,
        image_large_url -> Varchar,
        geo_spatial_url -> Nullable<Varchar>,
        contract_address -> Numeric,
        mint_fund_contract_address -> Nullable<Numeric>,
        p2p_trade_contract_address -> Nullable<Numeric>,
        shares_available -> Int4,
        offering_doc_link -> Nullable<Varchar>,
        property_media_header -> Varchar,
        property_media_footer -> Varchar,
        latest_price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        notification_id -> Nullable<Uuid>,
        notification_cognito_user_id -> Nullable<Varchar>,
        legal_contract_signed -> Nullable<Uuid>,
        legal_contract_signer -> Nullable<Varchar>,
        project_token_is_paused -> Bool,
        project_token_metadata_url -> Varchar,
        project_token_holder_address -> Nullable<Varchar>,
        project_token_frozen_balance -> Nullable<Numeric>,
        project_token_un_frozen_balance -> Nullable<Numeric>,
        mint_fund_rate -> Numeric,
        mint_fund_state -> Int4,
        mint_fund_token_contract_address -> Numeric,
        mint_fund_token_id -> Numeric,
        mint_fund_token_is_paused -> Bool,
        mint_fund_token_metadata_url -> Varchar,
        mint_fund_token_holder_address -> Nullable<Varchar>,
        mint_fund_token_frozen_balance -> Nullable<Numeric>,
        mint_fund_token_un_frozen_balance -> Nullable<Numeric>,
        p2p_trading_contract_address -> Numeric,
        p2p_trading_rate -> Nullable<Numeric>,
        p2p_trading_token_amount -> Nullable<Numeric>,
        p2p_trading_trader_address -> Nullable<Varchar>,
        holder_rewards -> Jsonb,
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = forest_project_user_view)]
#[diesel(primary_key(
    id,
    contract_address,
    notification_cognito_user_id,
    legal_contract_signer,
    project_token_holder_address
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub label: String,
    pub desc_short: String,
    pub desc_long: String,
    pub area: String,
    pub carbon_credits: i32,
    pub roi_percent: f32,
    pub state: ForestProjectState,
    pub image_large_url: String,
    pub geo_spatial_url: Option<String>,
    pub contract_address: Decimal,
    pub mint_fund_contract_address: Option<Decimal>,
    pub p2p_trade_contract_address: Option<Decimal>,
    pub shares_available: i32,
    pub offering_doc_link: Option<String>,
    pub property_media_header: String,
    pub property_media_footer: String,
    /// EuroE token amount without decimals
    pub latest_price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub notification_id: Option<uuid::Uuid>,
    pub notification_cognito_user_id: Option<String>,
    pub legal_contract_signed: Option<uuid::Uuid>,
    pub legal_contract_signer: Option<String>,
    pub project_token_is_paused: bool,
    pub project_token_metadata_url: String,
    pub project_token_holder_address: Option<String>,
    pub project_token_frozen_balance: Option<Decimal>,
    pub project_token_un_frozen_balance: Option<Decimal>,
    pub mint_fund_rate: Decimal,
    pub mint_fund_state: SecurityMintFundState,
    pub mint_fund_token_contract_address: Decimal,
    pub mint_fund_token_id: Decimal,
    pub mint_fund_token_is_paused: bool,
    pub mint_fund_token_metadata_url: String,
    pub mint_fund_token_holder_address: Option<String>,
    pub mint_fund_token_frozen_balance: Option<Decimal>,
    pub mint_fund_token_un_frozen_balance: Option<Decimal>,
    pub p2p_trading_contract_address: Decimal,
    pub p2p_trading_rate: Option<Decimal>,
    pub p2p_trading_token_amount: Option<Decimal>,
    pub p2p_trading_trader_address: Option<String>,
    pub holder_rewards: serde_json::Value,
}

impl ForestProjectUser {
    pub fn find(
        conn: &mut DbConn,
        id: Uuid,
        user_cognito_id: &str,
        user_account: Option<AccountAddress>,
    ) -> QueryResult<Option<Self>> {
        let user_account = user_account.map(|a| a.to_string()).unwrap_or_default();
        let project =
            forest_project_user_view::table
                .filter(forest_project_user_view::id.eq(id))
                .filter(
                    forest_project_user_view::notification_cognito_user_id
                        .is_null()
                        .or(forest_project_user_view::notification_cognito_user_id
                            .eq(user_cognito_id))
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::legal_contract_signer
                                .is_null()
                                .or(forest_project_user_view::legal_contract_signer
                                    .eq(user_cognito_id)),
                        ),
                )
                .select(ForestProjectUser::as_select())
                .get_result(conn)
                .optional()?;

        Ok(project)
    }

    pub fn list(
        conn: &mut DbConn,
        user_cognito_id: &str,
        user_account: Option<AccountAddress>,
        mint_fund_state: SecurityMintFundState,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let user_account = user_account.map(|a| a.to_string()).unwrap_or_default();
        let projects =
            forest_project_user_view::table
                .filter(
                    forest_project_user_view::notification_cognito_user_id
                        .is_null()
                        .or(forest_project_user_view::notification_cognito_user_id
                            .eq(user_cognito_id))
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::legal_contract_signer
                                .is_null()
                                .or(forest_project_user_view::legal_contract_signer
                                    .eq(user_cognito_id)),
                        )
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::mint_fund_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::mint_fund_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::p2p_trading_trader_address
                                .is_null()
                                .or(forest_project_user_view::p2p_trading_trader_address
                                    .eq(user_account.to_owned())),
                        ),
                )
                .filter(forest_project_user_view::mint_fund_state.eq(mint_fund_state))
                .filter(forest_project_user_view::state.eq(ForestProjectState::Listed))
                .select(ForestProjectUser::as_select())
                .order(forest_project_user_view::created_at.desc())
                .limit(page_size)
                .offset(page * page_size)
                .get_results(conn)?;
        let total_count =
            forest_project_user_view::table
                .filter(
                    forest_project_user_view::notification_cognito_user_id
                        .is_null()
                        .or(forest_project_user_view::notification_cognito_user_id
                            .eq(user_cognito_id))
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::legal_contract_signer
                                .is_null()
                                .or(forest_project_user_view::legal_contract_signer
                                    .eq(user_cognito_id)),
                        )
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::mint_fund_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::mint_fund_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::p2p_trading_trader_address
                                .is_null()
                                .or(forest_project_user_view::p2p_trading_trader_address
                                    .eq(user_account.to_owned())),
                        ),
                )
                .filter(forest_project_user_view::mint_fund_state.eq(mint_fund_state))
                .filter(forest_project_user_view::state.eq(ForestProjectState::Listed))
                .count()
                .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn list_owned(
        conn: &mut DbConn,
        user_cognito_id: &str,
        user_account: AccountAddress,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let user_account = user_account.to_string();
        let projects =
            forest_project_user_view::table
                .filter(
                    forest_project_user_view::notification_cognito_user_id
                        .is_null()
                        .or(forest_project_user_view::notification_cognito_user_id
                            .eq(user_cognito_id))
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::legal_contract_signer
                                .is_null()
                                .or(forest_project_user_view::legal_contract_signer
                                    .eq(user_cognito_id)),
                        )
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::mint_fund_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::mint_fund_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::p2p_trading_trader_address
                                .is_null()
                                .or(forest_project_user_view::p2p_trading_trader_address
                                    .eq(user_account.to_owned())),
                        ),
                )
                .filter(forest_project_user_view::state.eq(ForestProjectState::Listed))
                .filter(
                    forest_project_user_view::project_token_un_frozen_balance
                        .is_not_null()
                        .and(
                            forest_project_user_view::project_token_un_frozen_balance
                                .gt(Decimal::ZERO),
                        ),
                )
                .select(ForestProjectUser::as_select())
                .order(forest_project_user_view::created_at.desc())
                .limit(page_size)
                .offset(page * page_size)
                .get_results(conn)?;
        let total_count =
            forest_project_user_view::table
                .filter(
                    forest_project_user_view::notification_cognito_user_id
                        .is_null()
                        .or(forest_project_user_view::notification_cognito_user_id
                            .eq(user_cognito_id))
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::legal_contract_signer
                                .is_null()
                                .or(forest_project_user_view::legal_contract_signer
                                    .eq(user_cognito_id)),
                        )
                        .and(
                            forest_project_user_view::project_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::project_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::mint_fund_token_holder_address
                                .is_null()
                                .or(forest_project_user_view::mint_fund_token_holder_address
                                    .eq(user_account.to_owned())),
                        )
                        .and(
                            forest_project_user_view::p2p_trading_trader_address
                                .is_null()
                                .or(forest_project_user_view::p2p_trading_trader_address
                                    .eq(user_account.to_owned())),
                        ),
                )
                .filter(forest_project_user_view::state.eq(ForestProjectState::Listed))
                .filter(
                    forest_project_user_view::project_token_un_frozen_balance
                        .is_not_null()
                        .and(
                            forest_project_user_view::project_token_un_frozen_balance
                                .gt(Decimal::ZERO),
                        ),
                )
                .count()
                .get_result::<i64>(conn)?;

        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((projects, page_count))
    }

    pub fn holder_rewards_parsed(
        &self,
    ) -> Result<Vec<ForestProjectUserHolderReward>, serde_json::Error> {
        let rewards: Vec<ForestProjectUserHolderReward> =
            serde_json::from_value(self.holder_rewards.clone())?;
        Ok(rewards)
    }
}

#[derive(Object, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForestProjectUserHolderReward {
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id:       Decimal,
    pub total_frozen_reward:     Decimal,
    pub total_un_frozen_reward:  Decimal,
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
)]
#[diesel(table_name = crate::schema::forest_project_property_media)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectMedia {
    #[serde(skip_serializing)]
    pub id:         uuid::Uuid,
    pub image_url:  String,
    #[serde(skip_serializing)]
    pub project_id: uuid::Uuid,
}

impl ForestProjectMedia {
    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let media = schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::project_id.eq(project_id))
            .select(ForestProjectMedia::as_select())
            .order(schema::forest_project_property_media::created_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;
        let total_count = schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((media, page_count))
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProjectMedia> {
        let media = diesel::insert_into(schema::forest_project_property_media::table)
            .values(self)
            .returning(ForestProjectMedia::as_returning())
            .get_result(conn)?;
        Ok(media)
    }

    pub fn delete(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<ForestProjectMedia> {
        diesel::delete(schema::forest_project_property_media::table.find(id))
            .returning(ForestProjectMedia::as_returning())
            .get_result(conn)
    }

    pub fn delete_self(&self, conn: &mut DbConn) -> DbResult<ForestProjectMedia> {
        ForestProjectMedia::delete(conn, self.id)
    }

    pub fn find(conn: &mut DbConn, id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_project_property_media::table
            .filter(schema::forest_project_property_media::id.eq(id))
            .select(ForestProjectMedia::as_select())
            .first::<Self>(conn)
            .optional()
    }
}

#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[diesel(sql_type = schema::sql_types::ForestProjectState)]
#[derive(Enum)]
pub enum ForestProjectState {
    Draft,
    Listed,
    Archived,
}

impl FromSql<schema::sql_types::ForestProjectState, diesel::pg::Pg> for ForestProjectState {
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"draft" => Ok(ForestProjectState::Draft),
            b"listed" => Ok(ForestProjectState::Listed),
            b"archived" => Ok(ForestProjectState::Archived),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<schema::sql_types::ForestProjectState, diesel::pg::Pg> for ForestProjectState {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let s = match self {
            ForestProjectState::Draft => "draft",
            ForestProjectState::Listed => "listed",
            ForestProjectState::Archived => "archived",
        };
        out.write_all(s.as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = crate::schema::forest_project_prices)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id, price_at))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectPrice {
    pub project_id: uuid::Uuid,
    pub price:      Decimal,
    pub price_at:   NaiveDateTime,
}

impl ForestProjectPrice {
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ForestProjectPrice> {
        let price = diesel::insert_into(schema::forest_project_prices::table)
            .values(self)
            .returning(ForestProjectPrice::as_returning())
            .get_result(conn)?;
        Ok(price)
    }

    pub fn latest(conn: &mut DbConn, project_id: uuid::Uuid) -> DbResult<Option<Self>> {
        schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .select(ForestProjectPrice::as_select())
            .order(schema::forest_project_prices::price_at.desc())
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        price_at: NaiveDateTime,
    ) -> DbResult<Option<Self>> {
        schema::forest_project_prices::table
            .filter(
                schema::forest_project_prices::project_id
                    .eq(project_id)
                    .and(schema::forest_project_prices::price_at.eq(price_at)),
            )
            .select(ForestProjectPrice::as_select())
            .first::<Self>(conn)
            .optional()
    }

    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let prices = schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .select(ForestProjectPrice::as_select())
            .order(schema::forest_project_prices::price_at.desc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = schema::forest_project_prices::table
            .filter(schema::forest_project_prices::project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((prices, page_count))
    }

    pub fn delete(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        price_at: NaiveDateTime,
    ) -> DbResult<usize> {
        diesel::delete(
            schema::forest_project_prices::table.filter(
                schema::forest_project_prices::project_id
                    .eq(project_id)
                    .and(schema::forest_project_prices::price_at.eq(price_at)),
            ),
        )
        .execute(conn)
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
)]
#[diesel(table_name = crate::schema::forest_project_legal_contract_user_signatures)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LegalContractUserSignature {
    project_id:      uuid::Uuid,
    cognito_user_id: String,
    user_account:    String,
    user_signature:  String,
    created_at:      NaiveDateTime,
    updated_at:      NaiveDateTime,
}

impl LegalContractUserSignature {
    pub fn find(conn: &mut DbConn, id: Uuid, user_id: &str) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contract_user_signatures::dsl::*;
        forest_project_legal_contract_user_signatures
            .filter(project_id.eq(id).and(cognito_user_id.eq(user_id)))
            .first::<Self>(conn)
            .optional()
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
)]
#[diesel(table_name = crate::schema::forest_project_legal_contracts)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LegalContract {
    project_id: uuid::Uuid,
    text_url:   String,
    edoc_url:   String,
    pdf_url:    String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl LegalContract {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<Self>> {
        use crate::schema::forest_project_legal_contracts::dsl::*;
        let contract = forest_project_legal_contracts
            .filter(project_id.eq(id))
            .first::<Self>(conn)?;
        Ok(Some(contract))
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Associations,
    Serialize,
)]
#[diesel(table_name = crate::schema::forest_project_notifications)]
#[diesel(belongs_to(ForestProject, foreign_key = project_id))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id:              uuid::Uuid,
    pub project_id:      uuid::Uuid,
    pub cognito_user_id: String,
    pub created_at:      NaiveDateTime,
    pub updated_at:      NaiveDateTime,
}

impl Notification {
    pub fn exists(conn: &mut DbConn, project_id: Uuid, cognito_user_id: &str) -> DbResult<bool> {
        select(exists(
            schema::forest_project_notifications::table.filter(
                schema::forest_project_notifications::project_id
                    .eq(project_id)
                    .and(schema::forest_project_notifications::cognito_user_id.eq(cognito_user_id)),
            ),
        ))
        .get_result(conn)
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: Uuid,
        cognito_user_id: &str,
    ) -> DbResult<Option<Self>> {
        let res = schema::forest_project_notifications::table
            .filter(
                schema::forest_project_notifications::project_id
                    .eq(project_id)
                    .and(schema::forest_project_notifications::cognito_user_id.eq(cognito_user_id)),
            )
            .first(conn)
            .optional()?;

        Ok(res)
    }
}

diesel::table! {
    forest_project_holder_rewards_agg_view (id, holder_address, rewarded_token_contract, rewarded_token_id) {
        id -> Uuid,
        contract_address -> Numeric,
        holder_address -> Varchar,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        total_un_frozen_reward -> Numeric,
        total_frozen_reward -> Numeric
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = forest_project_holder_rewards_agg_view)]
#[diesel(primary_key(
    id,
    contract_address,
    holder_address,
    rewarded_token_contract,
    rewarded_token_id
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectHolderRewardAggregate {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub holder_address: String,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id: Decimal,
    pub total_un_frozen_reward: Decimal,
    pub total_frozen_reward: Decimal,
}

diesel::table! {
    forest_project_holder_rewards_view (id, contract_address, token_id, holder_address, rewarded_token_contract, rewarded_token_id) {
        id -> Uuid,
        contract_address -> Numeric,
        token_id -> Numeric,
        holder_address -> Varchar,
        frozen_balance -> Numeric,
        un_frozen_balance -> Numeric,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        frozen_reward -> Numeric,
        un_frozen_reward -> Numeric
    }
}

/// Claimable rewards for a holder
/// After claiming more rewards might be added
#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = forest_project_holder_rewards_view)]
#[diesel(primary_key(
    id,
    contract_address,
    token_id,
    holder_address,
    rewarded_token_contract,
    rewarded_token_id
))]
pub struct HolderReward {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub token_id: Decimal,
    pub holder_address: String,
    pub frozen_balance: Decimal,
    pub un_frozen_balance: Decimal,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id: Decimal,
    pub frozen_reward: Decimal,
    pub un_frozen_reward: Decimal,
}

impl HolderReward {
    pub fn list(conn: &mut DbConn, holder_address: &str) -> DbResult<Vec<Self>> {
        forest_project_holder_rewards_view::table
            .filter(forest_project_holder_rewards_view::holder_address.eq(holder_address))
            .select(HolderReward::as_select())
            .get_results(conn)
    }
}

diesel::table! {
    forest_project_holder_rewards_total_view (holder_address, rewarded_token_contract, rewarded_token_id) {
        holder_address -> Varchar,
        rewarded_token_contract -> Numeric,
        rewarded_token_id -> Numeric,
        total_un_frozen_reward -> Numeric,
        total_frozen_reward -> Numeric
    }
}

#[derive(
    Object,
    Selectable,
    Queryable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = forest_project_holder_rewards_total_view)]
#[diesel(primary_key(holder_address, rewarded_token_contract, rewarded_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectHolderRewardTotal {
    pub holder_address:          String,
    pub rewarded_token_contract: Decimal,
    pub rewarded_token_id:       Decimal,
    pub total_un_frozen_reward:  Decimal,
    pub total_frozen_reward:     Decimal,
}

impl ForestProjectHolderRewardTotal {
    pub fn list(conn: &mut DbConn, holder_address: &str) -> DbResult<Vec<Self>> {
        forest_project_holder_rewards_total_view::table
            .filter(forest_project_holder_rewards_total_view::holder_address.eq(holder_address))
            .select(ForestProjectHolderRewardTotal::as_select())
            .get_results(conn)
    }
}

diesel::table! {
    forest_project_investors_view (forest_project_id, investor) {
        cognito_user_id -> Varchar,
        email -> Varchar,
        account_address -> Varchar,
        forest_project_id -> Uuid,
        forest_project_contract_address -> Numeric,
        mint_fund_contract_address -> Numeric,
        investor -> Varchar,
        currency_amount -> Numeric,
        token_amount -> Numeric
    }
}
#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = forest_project_investors_view)]
#[diesel(primary_key(forest_project_id, investor))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectInvestor {
    pub cognito_user_id: String,
    pub email: String,
    pub account_address: String,
    pub forest_project_id: uuid::Uuid,
    pub forest_project_contract_address: Decimal,
    pub mint_fund_contract_address: Decimal,
    pub investor: String,
    pub currency_amount: Decimal,
    pub token_amount: Decimal,
}

impl ForestProjectInvestor {
    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let investors = forest_project_investors_view::table
            .filter(forest_project_investors_view::forest_project_id.eq(project_id))
            .select(ForestProjectInvestor::as_select())
            .order(forest_project_investors_view::investor.asc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = forest_project_investors_view::table
            .filter(forest_project_investors_view::forest_project_id.eq(project_id))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((investors, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        investor: &str,
    ) -> DbResult<Option<Self>> {
        forest_project_investors_view::table
            .filter(
                forest_project_investors_view::forest_project_id
                    .eq(project_id)
                    .and(forest_project_investors_view::investor.eq(investor)),
            )
            .select(ForestProjectInvestor::as_select())
            .first(conn)
            .optional()
    }
}

diesel::table! {
    forest_project_seller_view (forest_project_id, trader_address) {
        forest_project_id -> Uuid,
        forest_project_state -> crate::schema::sql_types::ForestProjectState,
        p2p_trade_contract_address -> Numeric,
        currency_token_id -> Numeric,
        currency_token_contract_address -> Numeric,
        trader_address -> Varchar,
        token_amount -> Numeric,
        rate -> Numeric,
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = forest_project_seller_view)]
#[diesel(primary_key(forest_project_id, trader_address))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ForestProjectSeller {
    pub forest_project_id: uuid::Uuid,
    pub forest_project_state: ForestProjectState,
    pub p2p_trade_contract_address: Decimal,
    pub currency_token_id: Decimal,
    pub currency_token_contract_address: Decimal,
    pub trader_address: String,
    pub token_amount: Decimal,
    pub rate: Decimal,
}

impl ForestProjectSeller {
    pub fn list(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        min_token_amount: Decimal,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let sellers = forest_project_seller_view::table
            .filter(forest_project_seller_view::forest_project_id.eq(project_id))
            .filter(forest_project_seller_view::token_amount.ge(min_token_amount))
            .select(ForestProjectSeller::as_select())
            .order(forest_project_seller_view::rate.asc())
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;

        let total_count = forest_project_seller_view::table
            .filter(forest_project_seller_view::forest_project_id.eq(project_id))
            .filter(forest_project_seller_view::token_amount.ge(min_token_amount))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        Ok((sellers, page_count))
    }

    pub fn find(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        trader_address: &str,
    ) -> DbResult<Option<Self>> {
        forest_project_seller_view::table
            .filter(
                forest_project_seller_view::forest_project_id
                    .eq(project_id)
                    .and(forest_project_seller_view::trader_address.eq(trader_address)),
            )
            .select(ForestProjectSeller::as_select())
            .first(conn)
            .optional()
    }
}
