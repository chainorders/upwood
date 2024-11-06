use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, NaiveDateTime, Utc};
use concordium_rust_sdk::base::hashes::{ModuleReference, TransactionHash};
use concordium_rust_sdk::base::smart_contracts::OwnedContractName;
use concordium_rust_sdk::id::types::AccountAddress;
use concordium_rust_sdk::types::queries::BlockInfo;
use concordium_rust_sdk::types::{AbsoluteBlockHeight, ContractAddress, TransactionIndex};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::dsl::*;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::sql_types::Integer;
use num_traits::ToPrimitive;
use poem_openapi::{Enum, Object};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::db::{DbConn, DbResult};
use tracing::instrument;

use crate::schema::{
    self, listener_config, listener_contract_calls, listener_contracts, listener_transactions,
};

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(table_name = schema::listener_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerConfig {
    pub id:                   i32,
    pub last_block_height:    Decimal,
    pub last_block_hash:      Vec<u8>,
    pub last_block_slot_time: NaiveDateTime,
}

impl ListenerConfig {
    /// Retrieves the last processed block from the database.
    #[instrument(skip_all)]
    pub fn find_last(
        conn: &mut DbConn,
    ) -> Result<Option<AbsoluteBlockHeight>, diesel::result::Error> {
        let config = listener_config::table
            .order(listener_config::last_block_height.desc())
            .limit(1)
            .select(listener_config::last_block_height)
            .first(conn)
            .optional()?
            .map(|block_height: Decimal| AbsoluteBlockHeight {
                height: block_height
                    .to_u64()
                    .expect("Block height should convert to u64"),
            });

        Ok(config)
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::listener_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerConfigInsert {
    pub last_block_height:    Decimal,
    pub last_block_hash:      Vec<u8>,
    pub last_block_slot_time: NaiveDateTime,
}

impl ListenerConfigInsert {
    /// Updates the last processed block in the database.
    #[instrument(skip_all, fields(block_height = %self.last_block_height))]
    pub fn insert(&self, conn: &mut DbConn) -> Result<Option<i32>, diesel::result::Error> {
        let created_id = insert_into(listener_config::table)
            .values(self)
            .on_conflict_do_nothing()
            .returning(listener_config::id)
            .get_result(conn)
            .optional()?;

        Ok(created_id)
    }
}

impl From<&BlockInfo> for ListenerConfigInsert {
    fn from(block: &BlockInfo) -> Self {
        Self {
            last_block_height:    block.block_height.height.into(),
            last_block_hash:      block.block_hash.to_vec(),
            last_block_slot_time: block.block_slot_time.naive_utc(),
        }
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Serialize, Object)]
#[diesel(primary_key(contract_address))]
#[diesel(table_name = schema::listener_contracts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerContract {
    pub module_ref:       String,
    pub contract_name:    String,
    pub contract_address: Decimal,
    pub owner:            String,
    pub processor_type:   ProcessorType,
    pub created_at:       NaiveDateTime,
}

impl ListenerContract {
    pub fn new(
        contract_address: Decimal,
        origin_ref: &ModuleReference,
        owner: &AccountAddress,
        init_name: &OwnedContractName,
        processor_type: ProcessorType,
        block_slot_time: DateTime<Utc>,
    ) -> Self {
        ListenerContract {
            contract_address,
            contract_name: init_name.to_string(),
            module_ref: origin_ref.to_string(),
            owner: owner.to_string(),
            processor_type,
            created_at: block_slot_time.naive_utc(),
        }
    }

    pub fn module_ref(&self) -> ModuleReference {
        self.module_ref.parse().expect("Invalid module ref")
    }

    pub fn contract_name(&self) -> OwnedContractName {
        OwnedContractName::new_unchecked(self.contract_name.to_owned())
    }

    pub fn contract_address(&self) -> ContractAddress {
        ContractAddress::new(
            self.contract_address
                .to_u64()
                .expect("Error converting contract address to u64"),
            0,
        )
    }

    #[instrument(skip_all, fields(contract_address = %contract_address))]
    pub fn find(
        conn: &mut DbConn,
        contract_address: Decimal,
    ) -> DbResult<Option<ListenerContract>> {
        let contract = listener_contracts::table
            .filter(listener_contracts::contract_address.eq(contract_address))
            .select(ListenerContract::as_select())
            .get_result(conn)
            .optional()?;
        Ok(contract)
    }

    #[instrument(skip_all, fields(contract_address = %self.contract_address, origin_ref = %origin_ref))]
    pub fn update_module_ref(
        &self,
        conn: &mut DbConn,
        origin_ref: &ModuleReference,
    ) -> DbResult<Self> {
        let contract = diesel::update(listener_contracts::table)
            .filter(listener_contracts::contract_address.eq(self.contract_address))
            .set(listener_contracts::module_ref.eq(origin_ref.to_string()))
            .returning(Self::as_returning())
            .get_result(conn)?;

        Ok(contract)
    }

    /// Adds a contract to the database.
    #[instrument(skip_all, fields(address = %self.contract_address, init_name = %self.contract_name, owner = %self.owner))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<Self> {
        let listener = insert_into(listener_contracts::table)
            .values(self)
            .returning(Self::as_returning())
            .get_result::<Self>(conn)?;

        Ok(listener)
    }
}

/// The processor type of a contract.
/// The processor type is used to determine which processor to use when processing events.
/// ### Caution! This is persisted to the database. Hence changing the processor type requires a migration.
#[repr(i32)]
#[derive(
    FromSqlRow,
    Debug,
    AsExpression,
    Clone,
    Copy,
    Enum,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[diesel(sql_type = Integer)]
pub enum ProcessorType {
    IdentityRegistry   = 1,
    SecuritySftSingle  = 2,
    SecuritySftRewards = 3,
    NftMultiRewarded   = 4,
    SecurityMintFund   = 5,
}

impl Display for ProcessorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorType::IdentityRegistry => write!(f, "IdentityRegistry"),
            ProcessorType::SecuritySftSingle => write!(f, "SecuritySftSingle"),
            ProcessorType::SecuritySftRewards => write!(f, "SecuritySftRewards"),
            ProcessorType::NftMultiRewarded => write!(f, "NftMultiRewarded"),
            ProcessorType::SecurityMintFund => write!(f, "SecurityMintFund"),
        }
    }
}

impl FromSql<Integer, diesel::pg::Pg> for ProcessorType {
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        Ok(match value {
            1 => ProcessorType::IdentityRegistry,
            2 => ProcessorType::SecuritySftSingle,
            3 => ProcessorType::SecuritySftRewards,
            4 => ProcessorType::NftMultiRewarded,
            5 => ProcessorType::SecurityMintFund,
            _ => return Err(format!("Invalid processor type: {}", value).into()),
        })
    }
}

impl ToSql<Integer, diesel::pg::Pg> for ProcessorType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let v = *self as i32;
        <i32 as ToSql<Integer, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::listener_contract_calls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerContractCall {
    pub id:               i64,
    pub transaction_hash: Vec<u8>,
    pub contract_address: Decimal,
    pub entrypoint_name:  String,
    pub ccd_amount:       Decimal,
    pub instigator:       String,
    pub sender:           String,
    pub events_count:     i32,
    pub call_type:        i32,
    pub is_processed:     bool,
    pub created_at:       NaiveDateTime,
}

impl ListenerContractCall {
    pub fn update_processed(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::update(listener_contract_calls::table)
            .filter(listener_contract_calls::id.eq(self.id))
            .set(listener_contract_calls::is_processed.eq(true))
            .execute(conn)?;
        Ok(())
    }
}

#[repr(i32)]
#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy)]
#[diesel(sql_type = Integer)]
pub enum CallType {
    Init     = 0,
    Update   = 1,
    Upgraded = 2,
}

impl FromSql<Integer, diesel::pg::Pg> for CallType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        match value {
            0 => Ok(CallType::Init),
            1 => Ok(CallType::Update),
            2 => Ok(CallType::Upgraded),
            _ => Err(format!("Unknown call type: {}", value).into()),
        }
    }
}

impl ToSql<Integer, diesel::pg::Pg> for CallType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let v = *self as i32;
        <i32 as ToSql<Integer, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::listener_contract_calls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerContractCallInsert<'a> {
    pub transaction_hash: Vec<u8>,
    pub contract_address: Decimal,
    pub entrypoint_name:  &'a str,
    pub ccd_amount:       Decimal,
    pub instigator:       &'a str,
    pub sender:           &'a str,
    pub events_count:     i32,
    pub call_type:        CallType,
    pub created_at:       NaiveDateTime,
}

impl<'a> ListenerContractCallInsert<'a> {
    #[instrument(skip_all, fields(contract_address = %self.contract_address, entrypoint_name = %self.entrypoint_name, instigator = %self.instigator, sender = %self.sender))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<ListenerContractCall> {
        let inserted = insert_into(listener_contract_calls::table)
            .values(self)
            .returning(ListenerContractCall::as_returning())
            .get_result(conn)?;
        Ok(inserted)
    }
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug)]
#[diesel(primary_key(transaction_hash))]
#[diesel(table_name = schema::listener_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerTransaction {
    pub block_hash:        Vec<u8>,
    pub block_height:      Decimal,
    pub block_slot_time:   NaiveDateTime,
    pub transaction_hash:  String,
    pub transaction_index: Decimal,
}

impl ListenerTransaction {
    pub fn new(block: &BlockInfo, txn_hash: TransactionHash, txn_index: TransactionIndex) -> Self {
        Self {
            block_hash:        block.block_hash.to_vec(),
            block_height:      block.block_height.height.into(),
            block_slot_time:   block.block_slot_time.naive_utc(),
            transaction_hash:  txn_hash.to_string(),
            transaction_index: txn_index.index.into(),
        }
    }

    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(listener_transactions::table)
            .values(self)
            .on_conflict(listener_transactions::transaction_hash)
            .do_nothing()
            .execute(conn)?;
        Ok(())
    }
}
