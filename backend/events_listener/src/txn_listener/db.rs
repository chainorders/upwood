use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::{ModuleReference, TransactionHash};
use concordium_rust_sdk::base::smart_contracts::OwnedContractName;
use concordium_rust_sdk::types::queries::BlockInfo;
use concordium_rust_sdk::types::{AbsoluteBlockHeight, TransactionIndex};
use concordium_rwa_backend_shared::db::DbConn;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::dsl::*;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::sql_types::Integer;
use num_traits::ToPrimitive;
use tracing::instrument;

use crate::schema::{
    self, listener_config, listener_contract_calls, listener_contracts, listener_transactions,
};

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(table_name = schema::listener_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerConfig {
    pub id:                i32,
    pub last_block_height: BigDecimal,
    pub last_block_hash:   Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::listener_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerConfigInsert {
    pub last_block_height: BigDecimal,
    pub last_block_hash:   Vec<u8>,
}

/// Retrieves the last processed block from the database.
#[instrument(skip(conn))]
pub fn get_last_processed_block(
    conn: &mut DbConn,
) -> Result<Option<AbsoluteBlockHeight>, diesel::result::Error> {
    let config = listener_config::table
        .order(listener_config::last_block_height.desc())
        .limit(1)
        .select(listener_config::last_block_height)
        .first(conn)
        .optional()?
        .map(|block_height: BigDecimal| AbsoluteBlockHeight {
            height: block_height
                .to_u64()
                .expect("Block height should convert to u64"),
        });

    Ok(config)
}

/// Updates the last processed block in the database.
#[instrument(skip(conn))]
pub fn update_last_processed_block(
    conn: &mut DbConn,
    block: &BlockInfo,
) -> Result<i32, diesel::result::Error> {
    let created_id: i32 = insert_into(listener_config::table)
        .values(ListenerConfigInsert {
            last_block_hash:   block.block_hash.bytes.to_vec(),
            last_block_height: block.block_height.height.into(),
        })
        .returning(listener_config::id)
        .get_result(conn)?;

    Ok(created_id)
}

#[derive(Selectable, Queryable, Identifiable, Insertable)]
#[diesel(primary_key(index))]
#[diesel(table_name = schema::listener_contracts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerContract {
    pub module_ref:    Vec<u8>,
    pub contract_name: String,
    pub index:         BigDecimal,
    pub sub_index:     BigDecimal,
    pub owner:         String,
}

/// Adds a contract to the database.
#[instrument(skip(conn))]
pub fn add_contract(
    conn: &mut DbConn,
    address: &concordium_rust_sdk::types::ContractAddress,
    origin_ref: &ModuleReference,
    init_name: &OwnedContractName,
    owner: &concordium_rust_sdk::id::types::AccountAddress,
) -> Result<(), diesel::result::Error> {
    insert_into(listener_contracts::table)
        .values(ListenerContract {
            index:         address.index.into(),
            sub_index:     address.subindex.into(),
            contract_name: init_name.to_string(),
            module_ref:    origin_ref.bytes.to_vec(),
            owner:         owner.to_string(),
        })
        .execute(conn)?;

    Ok(())
}

#[instrument(skip(conn))]
pub fn update_contract(
    conn: &mut DbConn,
    contract_address: &concordium_rust_sdk::types::ContractAddress,
    origin_ref: &ModuleReference,
) -> Result<(), diesel::result::Error> {
    diesel::update(listener_contracts::table)
        .filter(listener_contracts::index.eq::<BigDecimal>(contract_address.index.into()))
        .set(listener_contracts::module_ref.eq(origin_ref.bytes.to_vec()))
        .execute(conn)?;

    Ok(())
}

/// Finds a contract in the database based on its address.
#[instrument(skip(conn))]
pub fn find_contract(
    conn: &mut DbConn,
    contract_address: &concordium_rust_sdk::types::ContractAddress,
) -> Result<Option<(ModuleReference, OwnedContractName)>, diesel::result::Error> {
    let contract = listener_contracts::table
        .filter(listener_contracts::index.eq::<BigDecimal>(contract_address.index.into()))
        .select((
            listener_contracts::module_ref,
            listener_contracts::contract_name,
        ))
        .get_result(conn)
        .optional()?
        .map(|c: (Vec<u8>, String)| (to_module_ref(c.0), OwnedContractName::new_unchecked(c.1)));

    Ok(contract)
}

fn to_module_ref(vec: Vec<u8>) -> ModuleReference {
    ModuleReference::new(
        vec.as_slice()
            .try_into()
            .expect("Should convert vec to module ref"),
    )
}

#[derive(Selectable, Queryable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::listener_contract_calls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerContractCall {
    pub id:               i64,
    pub transaction_hash: Vec<u8>,
    pub index:            BigDecimal,
    pub sub_index:        BigDecimal,
    pub entrypoint_name:  String,
    pub ccd_amount:       BigDecimal,
    pub instigator:       String,
    pub sender:           String,
    pub events_count:     i32,
    pub call_type:        i32,
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
    pub index:            BigDecimal,
    pub sub_index:        BigDecimal,
    pub entrypoint_name:  &'a str,
    pub ccd_amount:       BigDecimal,
    pub instigator:       &'a str,
    pub sender:           &'a str,
    pub events_count:     i32,
    pub call_type:        CallType,
}

#[instrument(skip(conn))]
pub fn add_contract_call(
    conn: &mut DbConn,
    contract_call: ListenerContractCallInsert,
) -> Result<i64, diesel::result::Error> {
    let created_id: i64 = insert_into(listener_contract_calls::table)
        .values(contract_call)
        .returning(listener_contract_calls::id)
        .get_result(conn)?;
    Ok(created_id)
}

#[derive(Selectable, Queryable, Identifiable, Insertable, Debug)]
#[diesel(primary_key(transaction_hash))]
#[diesel(table_name = schema::listener_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListenerTransaction {
    pub block_hash:        Vec<u8>,
    pub block_height:      BigDecimal,
    pub block_slot_time:   NaiveDateTime,
    pub transaction_hash:  Vec<u8>,
    pub transaction_index: BigDecimal,
}

impl ListenerTransaction {
    pub fn new(block: &BlockInfo, txn_hash: TransactionHash, txn_index: TransactionIndex) -> Self {
        Self {
            block_hash:        block.block_hash.to_vec(),
            block_height:      block.block_height.height.into(),
            block_slot_time:   block.block_slot_time.naive_utc(),
            transaction_hash:  txn_hash.to_vec(),
            transaction_index: txn_index.index.into(),
        }
    }
}

#[instrument(skip(conn))]
pub fn upsert_transaction(
    conn: &mut DbConn,
    transaction: ListenerTransaction,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(listener_transactions::table)
        .values(transaction)
        .on_conflict(listener_transactions::transaction_hash)
        .do_nothing()
        .execute(conn)?;
    Ok(())
}
