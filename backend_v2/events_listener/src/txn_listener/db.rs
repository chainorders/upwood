use crate::{
    schema::{self, listener_config, listener_contracts},
};
use concordium_rwa_backend_shared::db::DbConn;
use bigdecimal::BigDecimal;
use concordium_rust_sdk::{
    base::{hashes::ModuleReference, smart_contracts::OwnedContractName},
    types::AbsoluteBlockHeight,
    v2::FinalizedBlockInfo,
};
use diesel::{dsl::*, prelude::*};
use num_traits::ToPrimitive;

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
pub fn get_last_processed_block(conn: &mut DbConn) -> anyhow::Result<Option<AbsoluteBlockHeight>> {
    let config = listener_config::table
        .order(listener_config::last_block_height.desc())
        .limit(1)
        .select(listener_config::last_block_height)
        .first(conn)
        .optional()?
        .map(|block_height: BigDecimal| AbsoluteBlockHeight {
            height: block_height.to_u64().expect("Block height should convert to u64"),
        });

    Ok(config)
}

/// Updates the last processed block in the database.
pub fn update_last_processed_block(
    conn: &mut DbConn,
    block: &FinalizedBlockInfo,
) -> anyhow::Result<i32> {
    let created_id: i32 = insert_into(listener_config::table)
        .values(ListenerConfigInsert {
            last_block_hash:   block.block_hash.bytes.to_vec(),
            last_block_height: block.height.height.into(),
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
}

/// Adds a contract to the database.
pub fn add_contract(
    conn: &mut DbConn,
    address: &concordium_rust_sdk::types::ContractAddress,
    origin_ref: &ModuleReference,
    init_name: &OwnedContractName,
) -> anyhow::Result<()> {
    insert_into(listener_contracts::table)
        .values(ListenerContract {
            index:         address.index.into(),
            sub_index:     address.subindex.into(),
            contract_name: init_name.to_string(),
            module_ref:    origin_ref.bytes.to_vec(),
        })
        .execute(conn)?;

    Ok(())
}

/// Finds a contract in the database based on its address.
pub fn find_contract(
    conn: &mut DbConn,
    contract_address: &concordium_rust_sdk::types::ContractAddress,
) -> anyhow::Result<Option<(ModuleReference, OwnedContractName)>> {
    let contract = listener_contracts::table
        .filter(listener_contracts::index.eq::<BigDecimal>(contract_address.index.into()))
        .select((listener_contracts::module_ref, listener_contracts::contract_name))
        .get_result(conn)
        .optional()?
        .map(|c: (Vec<u8>, String)| (to_module_ref(c.0), OwnedContractName::new_unchecked(c.1)));

    Ok(contract)
}

fn to_module_ref(vec: Vec<u8>) -> ModuleReference {
    ModuleReference::new(vec.as_slice().try_into().expect("Should convert vec to module ref"))
}
