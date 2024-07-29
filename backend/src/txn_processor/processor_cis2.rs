use chrono::{DateTime, Utc};
use concordium_cis2::{
    BurnEvent, Cis2Event, IsTokenAmount, IsTokenId, MintEvent, OperatorUpdate, TokenMetadataEvent,
    TransferEvent, UpdateOperatorEvent,
};
use concordium_rust_sdk::{
    base::contracts_common::{Cursor, Serial},
    cis2,
    types::{Address, ContractAddress},
};
use diesel::Connection;
use log::debug;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::shared::db::{DbConn, DbResult};

use super::db_security_cis2::{self as db};

pub fn cis2<T, A>(
    conn: &mut DbConn,
    now: chrono::DateTime<chrono::Utc>,
    cis2_address: &ContractAddress,
    event: Cis2Event<T, A>,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount + Serial, {
    match event {
        Cis2Event::Mint(MintEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_string().parse()?;
            let token_amount = to_cis2_token_amount(amount)?;
            conn.transaction(|conn| {
                db::insert_holder_or_add_balance(
                    conn,
                    &db::SecurityCis2TokenHolder::new(
                        cis2_address,
                        &token_id,
                        &owner,
                        &token_amount,
                        &cis2::TokenAmount(BigUint::zero()),
                        now,
                    ),
                )?;
                db::update_supply(conn, cis2_address, &token_id, &token_amount, true)?;
                DbResult::Ok(())
            })?
        }
        Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            db::insert_token_or_update_metadata(
                conn,
                &db::SecurityCis2Token::new(
                    cis2_address,
                    &token_id,
                    false,
                    metadata_url.url,
                    metadata_url.hash,
                    &cis2::TokenAmount(BigUint::zero()),
                    now,
                ),
            )?;
        }
        Cis2Event::Burn(BurnEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            let token_amount = to_cis2_token_amount(amount)?;
            conn.transaction(|conn| {
                db::update_sub_balance(conn, cis2_address, &token_id, &owner, &token_amount)?;
                db::update_supply(conn, cis2_address, &token_id, &token_amount, false)?;
                DbResult::Ok(())
            })?;
        }
        Cis2Event::Transfer(TransferEvent {
            token_id,
            from,
            to,
            amount,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            let token_amount = to_cis2_token_amount(amount)?;
            conn.transaction(|conn| {
                db::update_sub_balance(conn, cis2_address, &token_id, &from, &token_amount)?;
                db::insert_holder_or_add_balance(
                    conn,
                    &db::SecurityCis2TokenHolder::new(
                        cis2_address,
                        &token_id,
                        &to,
                        &token_amount,
                        &cis2::TokenAmount(BigUint::zero()),
                        now,
                    ),
                )?;
                DbResult::Ok(())
            })?;
        }
        Cis2Event::UpdateOperator(UpdateOperatorEvent {
            owner,
            operator,
            update,
        }) => {
            let record = db::SecurityCis2Operator::new(cis2_address, &owner, &operator);
            match update {
                OperatorUpdate::Add => db::insert_operator(conn, &record)?,
                OperatorUpdate::Remove => db::delete_operator(conn, &record)?,
            }
        }
    }
    Ok(())
}

pub fn agent_added(
    conn: &mut DbConn,
    agent: Address,
    now: DateTime<Utc>,
    cis2_address: &ContractAddress,
) -> anyhow::Result<()> {
    Ok(db::insert_agent(conn, db::Agent::new(agent, now, cis2_address))?)
}

pub fn agent_removed(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    agent: Address,
) -> anyhow::Result<()> {
    db::remove_agent(conn, cis2_address, &agent)?;
    Ok(())
}

pub fn compliance_updated(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    compliance_contract: ContractAddress,
) -> anyhow::Result<()> {
    db::upsert_compliance(
        conn,
        &db::SecurityCis2ContractCompliance::new(cis2_address, &compliance_contract),
    )?;
    Ok(())
}

pub fn identity_registry_updated(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    identity_registry_contract: ContractAddress,
) -> anyhow::Result<()> {
    db::upsert_identity_registry(
        conn,
        &db::SecurityCis2ContractIdentityRegistry::new(cis2_address, &identity_registry_contract),
    )?;
    Ok(())
}

pub fn token_paused<T>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString, {
    db::update_token_paused(conn, cis2_address, &to_cis2_token_id(token_id)?, true)?;
    Ok(())
}

pub fn token_unpaused<T>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString, {
    db::update_token_paused(conn, cis2_address, &to_cis2_token_id(token_id)?, false)?;
    Ok(())
}

pub fn account_recovered(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    lost_account: concordium_rust_sdk::types::Address,
    new_account: concordium_rust_sdk::types::Address,
) -> anyhow::Result<()> {
    let updated_rows = conn.transaction(|conn| {
        db::insert_recovery_record(
            conn,
            &db::SecurityCis2RecoveryRecord::new(cis2_address, &lost_account, &new_account),
        )?;
        db::update_replace_holder(conn, cis2_address, &lost_account, &new_account)
    })?;
    debug!("account recovery, {} token ids updated", updated_rows);
    Ok(())
}

pub fn token_frozen<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
    address: concordium_rust_sdk::types::Address,
    amount: A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::update_balance_frozen(
        conn,
        cis2_address,
        &to_cis2_token_id(token_id)?,
        &address,
        &to_cis2_token_amount(amount)?,
        true,
    )?;

    Ok(())
}

pub fn token_un_frozen<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
    address: concordium_rust_sdk::types::Address,
    amount: A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::update_balance_frozen(
        conn,
        cis2_address,
        &to_cis2_token_id(token_id)?,
        &address,
        &to_cis2_token_amount(amount)?,
        false,
    )?;

    Ok(())
}

fn to_cis2_token_amount<A>(amount: A) -> Result<cis2::TokenAmount, anyhow::Error>
where
    A: Serial, {
    let mut bytes = vec![];
    amount
        .serial(&mut bytes)
        .map_err(|_| anyhow::Error::msg("error serializing amount to bytes"))?;
    let mut cursor: Cursor<_> = Cursor::new(bytes);
    Ok(<cis2::TokenAmount as concordium_rust_sdk::base::contracts_common::Deserial>::deserial(
        &mut cursor,
    )?)
}

fn to_cis2_token_id<T>(token_id: T) -> Result<cis2::TokenId, anyhow::Error>
where
    T: IsTokenId + ToString, {
    let token_id = token_id.to_string().parse()?;
    Ok(token_id)
}
