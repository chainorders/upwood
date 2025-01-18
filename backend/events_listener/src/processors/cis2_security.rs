use core::fmt;

use chrono::NaiveDateTime;
use concordium_cis2::{
    BurnEvent, Cis2Event, IsTokenAmount, IsTokenId, MintEvent, OperatorUpdate, TokenMetadataEvent,
    TransferEvent, UpdateOperatorEvent,
};
use concordium_protocols::concordium_cis2_security::*;
use concordium_rust_sdk::base::contracts_common::{Deserial, Serial};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use diesel::Connection;
use rust_decimal::Decimal;
use shared::db::cis2_security::{
    Agent, Compliance, IdentityRegistry, Operator, RecoveryRecord, Token, TokenHolder,
    TokenHolderBalanceUpdate, TokenHolderBalanceUpdateType,
};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};
use uuid::Uuid;

use crate::processors::cis2_utils::*;
use crate::processors::ProcessorError;

#[allow(clippy::too_many_arguments)]
#[instrument(
    name="cis2",
    skip_all,
    fields(contract = %contract)
)]
pub fn process_events_cis2<T, A>(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    txn_sender: &str,
    txn_instigator: &str,
    contract: &ContractAddress,
    event: Cis2Event<T, A>,
) -> Result<(), ProcessorError>
where
    T: IsTokenId,
    A: IsTokenAmount+Serial,
{
    match event {
        Cis2Event::Mint(MintEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            Token::find(conn, contract.to_decimal(), token_id)?
                .ok_or(ProcessorError::TokenNotFound {
                    contract: contract.to_decimal(),
                    token_id,
                })
                .map(|mut token| {
                    token.supply += amount;
                    token.update_time = block_time;
                    token
                })?
                .update(conn)?;
            let holder =
                TokenHolder::find(conn, contract.to_decimal(), token_id, &owner.to_string())?
                    .map(|mut holder| {
                        holder.un_frozen_balance += amount;
                        holder.update_time = block_time;
                        holder
                    })
                    .unwrap_or_else(|| TokenHolder {
                        cis2_address: contract.to_decimal(),
                        holder_address: owner.to_string(),
                        token_id,
                        frozen_balance: Decimal::ZERO,
                        un_frozen_balance: amount,
                        update_time: block_time,
                        create_time: block_time,
                    })
                    .upsert(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: owner.to_string(),
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::Mint,
                create_time: block_time,
            }
            .insert(conn)?;
            info!(
                "Minted {} tokens of token_id {} to {}",
                amount,
                token_id,
                owner.to_string()
            );
            Ok(())
        }
        Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        }) => {
            Token::find(conn, contract.to_decimal(), token_id.to_decimal())?
                .map(|mut token| {
                    token.metadata_url = metadata_url.url.clone();
                    token.metadata_hash = metadata_url.hash.map(hex::encode);
                    token.update_time = block_time;
                    token
                })
                .unwrap_or_else(|| Token {
                    cis2_address:  contract.to_decimal(),
                    token_id:      token_id.to_decimal(),
                    metadata_url:  metadata_url.url.clone(),
                    metadata_hash: metadata_url.hash.map(hex::encode),
                    supply:        Decimal::ZERO,
                    is_paused:     false,
                    update_time:   block_time,
                    create_time:   block_time,
                })
                .upsert(conn)?;
            info!(
                "Updated metadata for token {}: {}",
                token_id.to_decimal(),
                metadata_url.url
            );
            Ok(())
        }
        Cis2Event::Burn(BurnEvent {
            token_id,
            owner,
            amount,
        }) => {
            let contract = contract.to_decimal();
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            Token::find(conn, contract, token_id)?
                .map(|mut token| {
                    token.supply -= amount;
                    token.update_time = block_time;
                    token
                })
                .ok_or(ProcessorError::TokenNotFound { contract, token_id })?
                .update(conn)?;
            let holder = TokenHolder::find(conn, contract, token_id, &owner.to_string())?
                .map(|mut holder| {
                    holder.un_frozen_balance -= amount;
                    holder.update_time = block_time;
                    holder
                })
                .ok_or(ProcessorError::TokenHolderNotFound {
                    contract,
                    token_id,
                    holder_address: owner.to_string(),
                })?
                .update(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract,
                token_id,
                holder_address: owner.to_string(),
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::Burn,
                create_time: block_time,
            }
            .insert(conn)?;
            info!(
                "Burned {} tokens of token_id {} from {}",
                amount,
                token_id,
                owner.to_string()
            );
            Ok(())
        }
        Cis2Event::Transfer(TransferEvent {
            token_id,
            from,
            to,
            amount,
        }) => {
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            let holder_from =
                TokenHolder::find(conn, contract.to_decimal(), token_id, &from.to_string())?
                    .map(|mut holder| {
                        holder.un_frozen_balance -= amount;
                        holder.update_time = block_time;
                        holder
                    })
                    .ok_or(ProcessorError::TokenHolderNotFound {
                        contract: contract.to_decimal(),
                        token_id,
                        holder_address: from.to_string(),
                    })?
                    .update(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: holder_from.holder_address,
                amount,
                frozen_balance: holder_from.frozen_balance,
                un_frozen_balance: holder_from.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::TransferOut,
                create_time: block_time,
            }
            .insert(conn)?;
            let holder_to =
                TokenHolder::find(conn, contract.to_decimal(), token_id, &to.to_string())?
                    .map(|mut holder| {
                        holder.un_frozen_balance += amount;
                        holder.update_time = block_time;
                        holder
                    })
                    .unwrap_or_else(|| TokenHolder {
                        cis2_address: contract.to_decimal(),
                        holder_address: to.to_string(),
                        token_id,
                        frozen_balance: Decimal::ZERO,
                        un_frozen_balance: amount,
                        update_time: block_time,
                        create_time: block_time,
                    })
                    .upsert(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: holder_to.holder_address,
                amount,
                frozen_balance: holder_to.frozen_balance,
                un_frozen_balance: holder_to.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::TransferIn,
                create_time: block_time,
            }
            .insert(conn)?;
            info!(
                "Transferred {} tokens of token_id {} from {} to {}",
                amount,
                token_id,
                from.to_string(),
                to.to_string()
            );
            Ok(())
        }
        Cis2Event::UpdateOperator(UpdateOperatorEvent {
            owner,
            operator,
            update,
        }) => {
            let record = Operator::new(contract.to_decimal(), &owner, &operator);
            match update {
                OperatorUpdate::Add => {
                    record.insert(conn)?;
                    info!(
                        "Added operator {} for owner {}",
                        operator.to_string(),
                        owner.to_string()
                    );
                }
                OperatorUpdate::Remove => {
                    record.delete(conn)?;
                    info!(
                        "Removed operator {} for owner {}",
                        operator.to_string(),
                        owner.to_string()
                    );
                }
            }
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn process_events<T, A, R>(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    txn_sender: &str,
    txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError>
where
    T: IsTokenId+fmt::Debug,
    A: IsTokenAmount+fmt::Debug,
    R: Deserial+fmt::Debug+std::string::ToString,
{
    for event in events {
        let parsed_event = event
            .parse::<Cis2SecurityEvent<T, A, R>>()
            .expect("Failed to parse event");
        trace!("{:#?}", parsed_event);

        process_parsed_event(
            conn,
            contract,
            parsed_event,
            block_time,
            block_height,
            txn_index,
            txn_sender,
            txn_instigator,
        )?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn process_parsed_event<T, A, R>(
    conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    contract: &ContractAddress,
    parsed_event: Cis2SecurityEvent<T, A, R>,
    block_time: NaiveDateTime,
    block_height: Decimal,
    txn_index: Decimal,
    txn_sender: &str,
    txn_instigator: &str,
) -> Result<(), ProcessorError>
where
    T: IsTokenId+fmt::Debug,
    A: IsTokenAmount+fmt::Debug,
    R: Deserial+fmt::Debug+std::string::ToString,
{
    match parsed_event {
        Cis2SecurityEvent::TokenRemoved(token_id) => {
            Token::find(conn, contract.to_decimal(), token_id.to_decimal())?
                .ok_or(ProcessorError::TokenNotFound {
                    contract: contract.to_decimal(),
                    token_id: token_id.to_decimal(),
                })?
                .delete(conn)?;
            info!("Removed token_id {}", token_id.to_decimal());
        }
        Cis2SecurityEvent::AgentAdded(AgentUpdatedEvent { agent, roles }) => {
            Agent::new(
                agent,
                block_time,
                contract.to_decimal(),
                roles.iter().map(|r| r.to_string()).collect(),
            )
            .insert(conn)?;
            info!(
                "Added agent {} with roles: {}",
                agent.to_string(),
                roles
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        Cis2SecurityEvent::AgentRemoved(AgentUpdatedEvent { agent, roles: _ }) => {
            Agent::delete(conn, contract.to_decimal(), &agent)?;
            info!("Removed agent {}", agent.to_string(),);
        }
        Cis2SecurityEvent::ComplianceAdded(ComplianceAdded(compliance_contract)) => {
            Compliance::new(contract.to_decimal(), compliance_contract.to_decimal())
                .upsert(conn)?;
            info!(
                "Added compliance contract {}",
                compliance_contract.to_string(),
            );
        }
        Cis2SecurityEvent::IdentityRegistryAdded(IdentityRegistryAdded(
            identity_registry_contract,
        )) => {
            IdentityRegistry::new(
                contract.to_decimal(),
                identity_registry_contract.to_decimal(),
            )
            .upsert(conn)?;
            info!(
                "Added identity registry contract {}",
                identity_registry_contract.to_string(),
            );
        }
        Cis2SecurityEvent::Paused(Paused { token_id }) => {
            Token::update_paused(conn, contract.to_decimal(), token_id.to_decimal(), true)?;
            info!("Paused token_id {}", token_id.to_decimal());
        }
        Cis2SecurityEvent::UnPaused(Paused { token_id }) => {
            Token::update_paused(conn, contract.to_decimal(), token_id.to_decimal(), false)?;
            info!("Unpaused token_id {}", token_id.to_decimal());
        }
        Cis2SecurityEvent::Recovered(RecoverEvent {
            lost_account,
            new_account,
        }) => {
            let updated_rows = conn.transaction(|conn| {
                RecoveryRecord::new(contract.to_decimal(), &lost_account, &new_account)
                    .insert(conn)?;
                TokenHolder::replace(conn, contract.to_decimal(), &lost_account, &new_account)
            })?;
            info!("account recovery, {} token ids updated", updated_rows);
        }
        Cis2SecurityEvent::TokenFrozen(TokenFrozen {
            address,
            amount,
            token_id,
        }) => {
            let holder = TokenHolder::update_balance_frozen(
                conn,
                contract.to_decimal(),
                token_id.to_decimal(),
                &address,
                amount.to_decimal(),
                true,
            )?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract.to_decimal(),
                token_id: token_id.to_decimal(),
                holder_address: address.to_string(),
                amount: amount.to_decimal(),
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::Freeze,
                create_time: block_time,
            }
            .insert(conn)?;
            info!(
                "Frozen {} tokens of token_id {} for {}",
                amount.to_decimal(),
                token_id.to_decimal(),
                address.to_string()
            );
        }
        Cis2SecurityEvent::TokenUnFrozen(TokenFrozen {
            address,
            amount,
            token_id,
        }) => {
            let holder = TokenHolder::update_balance_frozen(
                conn,
                contract.to_decimal(),
                token_id.to_decimal(),
                &address,
                amount.to_decimal(),
                false,
            )?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                id_serial: None,
                block_height,
                txn_index,
                cis2_address: contract.to_decimal(),
                token_id: token_id.to_decimal(),
                holder_address: address.to_string(),
                amount: amount.to_decimal(),
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                txn_sender: txn_sender.to_string(),
                txn_instigator: txn_instigator.to_string(),
                update_type: TokenHolderBalanceUpdateType::UnFreeze,
                create_time: block_time,
            }
            .insert(conn)?;
            info!(
                "Unfrozen {} tokens of token_id {} for {}",
                amount.to_decimal(),
                token_id.to_decimal(),
                address.to_string()
            );
        }
        Cis2SecurityEvent::Cis2(e) => process_events_cis2(
            conn,
            block_height,
            block_time,
            txn_index,
            txn_sender,
            txn_instigator,
            contract,
            e,
        )?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use concordium_cis2::{TokenAmountU64, TokenAmountU8};
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use crate::processors::cis2_utils::TokenAmountToDecimal;

    #[test]
    fn token_amount_conversions() {
        let amount = TokenAmountU8(0).to_decimal();
        assert_eq!(amount, Decimal::ZERO);
        assert_eq!(amount.to_string(), "0");

        let amount: u8 = 255;
        let token_amount = TokenAmountU8(amount).to_decimal();
        assert_eq!(token_amount, Decimal::from_u8(amount).unwrap());
        assert_eq!(token_amount.to_string(), "255");

        let amount: u64 = 255;
        let token_amount = TokenAmountU64(amount).to_decimal();
        assert_eq!(token_amount, Decimal::from_u64(amount).unwrap());
        assert_eq!(token_amount.to_string(), "255");
    }
}
